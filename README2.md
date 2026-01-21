## Circom篇
### 安装circom和snarkjs
请访问链接

> [https://docs.circom.io/getting-started/installation/](https://docs.circom.io/getting-started/installation/)
>

### 编写模版电路
`circom` 允许程序员定义算术电路的约束。所有约束必须采用 A*B + C = 0 的形式，其中 A、B 和 C 是信号的线性组合。让我们定义我们的第一个电路，它简单地将两个输入信号相乘并产生一个输出信号。

```plain
pragma circom 2.0.0;

/*This circuit template checks that c is the multiplication of a and b.*/
template Multiplier2 () {

  // Declaration of signals.
  signal input a;
  signal input b;
  signal output c;

  // Constraints.
  c <== a * b;

}
```

`pragma` 指令用于指定编译器版本

然后，我们使用关键字`template` 来定义新电路 `Multiplier2`，并指定他有`a, b, c`三个信号。其中，`a` 和 `b` 为 2 个 private 输入信号，名为 `a` 和 `b` ，还有一个输出信号 `c`。

输入和输出使用`<==`运算符进行关联。 在 circom 中，`<==`运算符做两件事。 首先是连接信号，第二个是施加约束（单纯赋值使用`<--`，单纯约束使用`===`）。在本例中，我们使用`<==`将`c`连接到`a`和`b`，同时将`c`约束为`a * b`的值，即电路做的事情是让强制信号 `c` 为 `a*b` 的值。

---

**注意**：**在Circom对于电路信号（输入或输出导线）是否私密的指定，与Rust代码中对于电路导线是否私密的指定，是无关的**。如`circoms/lrs/lrs.circom` 电路定义，其中`sk,phi,sc,L`都是输入信号。

```plain
pragma circom 2.2.0;

include "./include/escalarmul.circom";
include "./include/mimc.circom";



template Main() {
    signal input sk;
    signal input phi;
    signal input sc;
    signal input L;

    var base[2] = [
        5299619240641551281634865583518297030282874472190772894086521144482721001553,
        16950150798460657717958625567821834550301663161624707787222815936182638968203
    ];

    component n2b = Num2Bits(253);
    component PKComp = EscalarMul(253, base);

    PKComp.inp[0] <== 0;
    PKComp.inp[1] <== 1;

    var i;

    sk ==> n2b.in;

    for  (i=0; i<253; i++) {
        n2b.out[i] ==> PKComp.in[i];
    }

    component h_phi = MultiMiMC7(2, 91);
    h_phi.in[0] <== PKComp.out[0];
    h_phi.in[1] <== PKComp.out[1];
    h_phi.k <== 1;
    phi === h_phi.out;

    component h_L = MultiMiMC7(2, 91);
    h_L.in[0] <== sk;
    h_L.in[1] <== sc;
    h_L.k <== 1;
    L === h_L.out;
}

component main = Main();
```

若使用与circom配套的snarkjs，则这四者默认都是秘密值。但我们并不使用snarkjs证明，而是依赖我们的Rust代码实现，**在Rust代码中会重新定义电路输入输出导线信息**，如`src/constants.rs`定义：

```rust
// Information of LRS circuit for the CircDescriptor
pub const NUM_PUB_IO_LRS_SE: usize = 2;
pub const NUM_COMMIT_WITNESS_LRS_SE: usize = 1;
pub const IOPUTS_NAME_LRS_SE: [&str; 4] = ["main.sc", "main.L", "main.phi", "main.sk"];
pub const PATH_PREFIX_LRS_SE: &str = "./circoms/lrs/";
pub const CIRCUIT_NAME_LRS_SE: &str = "lrs";
```

+ 第四行我们定义了输入输出名称向量`IOPUTS_NAME_LRS_SE`。名称的排布规则是：公开输入（`"main.sc", "main.L"`）在前，需要承诺的秘密输入（`"main.phi"`）在中间，其他秘密输入在后（`"main.sk"`）
+ 随后我们在第二行制定了公开输入数量`NUM_PUB_IO_LRS_SE`为`2`，在第三行定义了需要承诺的秘密输入数量`NUM_COMMIT_WITNESS_LRS_SE`为`1`，其余为其他秘密输入。

---

### 构建实际电路
要构建**实际电路**，我们必须创建**模板电路**的一个实例（使用名为`main`的组件实例化它）。 为此，请创建一个包含以下内容的文件：

```plain
 pragma circom 2.0.0;

include "./Multiplier2.circom";

component main = Multiplier2();
```

### 编译电路
`circom multiplier2_inst.circom --r1cs --wasm --sym --json -p=bn128`

这条命令将会执行编译，并依据传递的参数生成特定文件

+ `-p=bn128`	指定电路基于`BN256`曲线（这俩名字好像是同一个东西）
+ `--r1cs`		若指定，将会生成R1CS矩阵文件 `**multiplier2_inst.r1cs**`，不可读
+ `--json`		R1CS矩阵的可读版本`**multiplier2_inst_constraints.json**`（我们的Rust代码依赖它构建约束）
+ `--sym`		符号表`**multiplier2_inst.sym**`，可读。其第1、4列提供**下标**与**导线名**的对应关系。_后续证据文件提供__**下标**__与__**值**__的对应关系_。
+   			还会生成一个`**multiplier2_inst_js**`目录，包含`multiplier2_inst.wasm`，`generate_witness.js`等生成证据所需的文件
+ `--wasm`		生成证据需要，`multiplier2_inst.wasm`。在`multiplier2_inst_js`目录下，不可读。

变异结束后，可以使用`snarkjs ri multiplier2_inst.r1cs`查看电路有关信息。如

```plain
[INFO]  snarkJS: Curve: bn-128
[INFO]  snarkJS: # of Wires: 4
[INFO]  snarkJS: # of Constraints: 1
[INFO]  snarkJS: # of Private Inputs: 2
[INFO]  snarkJS: # of Public Inputs: 0
[INFO]  snarkJS: # of Labels: 4
[INFO]  snarkJS: # of Outputs: 1
```

### 提供电路输入
我们需要为所有`signal input`类型的信号，提供对应的值，输入以`json`格式给出。以上述的`Multiplier2`电路为例，它需要信号`a,b`的值，穿件对应的输入文件`input.json`：

```json
{"a": 3, "b": 11}
```

将`input.json`放入`multiplier2_inst_js`目录下。

### 使用自定义的 generate_witness.js！
`generate_witness.js`是执行电路编译命令后，在`multiplier2_inst_js`目录下自动包含的一个脚本，用于生成证据。但默认`generate_witness.js`脚本生成的证据`witness.wtns`是不可读的。为此，我们将默认脚本封装了一下，能够生成可读的证据。脚本如下

```json
const wc  = require("./witness_calculator.js");
const { readFileSync, writeFile } = require("fs");

if (process.argv.length != 5) {
    console.log("Usage: node generate_witness.js <file.wasm> <input.json> <output.wtns>");
} else {
    const input = JSON.parse(readFileSync(process.argv[3], "utf8"));
    
    const buffer = readFileSync(process.argv[2]);
    wc(buffer).then(async witnessCalculator => {
        const w = await witnessCalculator.calculateWitness(input, 0);
        
        // Prepare data to write to JSON with BigInt conversion to string
        const outputData = {};
        for (let i = 0; i < w.length; i++) {
            outputData[i] = w[i].toString(); // Convert BigInt to string
        }

        // Save the outputData to a JSON file
        const jsonFileName = process.argv[4] + ".json";
        writeFile(jsonFileName, JSON.stringify(outputData, null, 2), (err) => {
            if (err) throw err;
            console.log(`Witness data saved to ${jsonFileName}`);
        });

        const buff = await witnessCalculator.calculateWTNSBin(input, 0);
        writeFile(process.argv[4], buff, function(err) {
            if (err) throw err;
        });
    });
}
```

将`generate_witness_backend.js`脚本拷贝到`multiplier2_inst_js`目录下，并重命名为`generate_witness.js`替换默认脚本

### 证据生成
`node generate_witness.js multiplier2_inst.wasm input.json witness.wtns`

生成证据，输出！

## 读取电路
### 读取约束
以电路为例，首先观察一下输出的的内容。注意我们的约束是`c <== a * b;`

> 21888242871839275222246405745257275088548364400416034343698204186575808495616 相当于 -1 
>

```json
{
"constraints": 
  [	// 很多约束
    [	// 一个约束 A * B = C
      {"2":"21888242871839275222246405745257275088548364400416034343698204186575808495616"},	// A 部分，-1 * 变量二(变量 a)
      {"3":"1"},	// B 部分，1 * 变量三(变量三 b)
      {"1":"21888242871839275222246405745257275088548364400416034343698204186575808495616"}		// C 部分，-1 * 变量一(变量 c)
    ] 
  ]
}
```

```json
1,1,0,main.c
2,2,0,main.a
3,3,0,main.b
```

结合变量与下标的对应关系，这意味着约束

`(-1 * a) * (1 * b) = (-1 * c)`

恰如电路描述

---

在 `src/lrs/structures.rs`文件中，我们定义了结构体`Constraints`来描述电路的约束，并为该结构体提供了`read_from_file`方法，从`*_constraints.json`文件中实际的约束信息，后续经由`generate_constraints(src/lrs/lrs_circ.rs:99-182,2.5节详谈)`生成约束后，交由我们的证明系统使用。

### 读取导线值
```json
{
  "0": "1",
  "1": "33",
  "2": "3",
  "3": "11"
}
```

观察`multiplier2_inst`电路的证据：下标`0`固定是常数`1`，下标`2,3`对应`input.json`中`a=3,b=11`，下标`1`对应导线`c=a*b=33`。

在`src/lrs/utils.rs`中，我们提供了`**read_witness_file**`方法，用于从`*_js/witness.wtns.json`文件中读取所有导线的值。以**下标**为键存在`hashMap`里

### 读取 sym 文件
在`src/lrs/utils.rs`中，我们提供了`**read_sym_file**`方法，读取并存储**下标**与**导线名**的映射

### 构造电路
在`src/lrs/structures.rs`中，我们定义了结构体`CircDescriptor`，存储电路的基本信息：电路所有`signal`中公开值数量、需要承诺值的数量，及对应的名称，电路相关文件的存放路径，以及电路名。依据`CircDescriptor`的信息，就可以进一步构造电路了。

```rust
pub struct CircDescriptor {
    pub num_pub_io: usize,
    pub num_commit_witness: usize,
    pub ioputs_name: Vec<String>,
    pub path_prefix: String,
    pub circuit_name: String,
}
```

---

我们在`src/lrs/lrs_circ.rs`中用结构体`LRSCirc`表示电路在代码中的表示。它有一个`construct`方法，接收一个`CircDescriptor`，然后依次读取约束、导线值`witness_data`、sym文件，并从`witness_data`中选出比较特殊的公开值部分`instance`和需要承诺的值`commit_witness`。

### 将读取的约束同步到 ark_relations::r1cs 表示
ark_relations::r1cs 是一个现有的 R1CS 库。在 ark_relations::r1cs 表示中，约束系统从下往上依次是 `变量-> 约束 -> 约束系统`，其中变量分为两类：`input_variable`表示公开值，`witness_variable`表示秘密值。

我们在对 ccGro16 的代码实现（`src/cc`）中，将所有导线按下标分成三部分：公开值，需要承诺的秘密值，其他秘密值。

为了直接利用 ark_relations::r1cs，我们在`generate_constraints(src/lrs/lrs_circ.rs:99-182)`声明“需要承诺的秘密值，其他秘密值”的变量时，都将其声明为`witness_variable`，但总是将“需要承诺的秘密值”放在“其他秘密值”前面（下标更小）。并通过修改`src/cc`证明和验证代码，特殊处理“需要承诺的秘密值”部分，实现生成其承诺的效果

## 认识 cc, link, sma, lrs
```markdown
├─ src/
│  ├─ bin/
│  │  ├─ lrs_a.rs       # test for LRS from snarks (implemented by using cc, and set num_commit_witness=0)
│  │  ├─ lrs_se.rs      # test for LRS with simulation extractable (SE) from sma + cc + link
│  ├─ cc/               # commit-carry snarks with SE 
│  ├─ link/             # link snark with one time SE from [1] Figure.6
│  ├─ lrs/              # combine sma + cc + link
│  ├─ sma/              # set membership argument with SE
```

代码的其他部分就是搭积木了。这些部分的代码几乎与论文方案直接相关，因此花费些时间熟悉方案，回来查看代码设计还是比较方便的。

+ src/bin/lrs_se.rs 针对不同大小的环，进行重复测试。依赖 src/lrs 中：sign mod 下 sign 接口和 verify mod 下 verify 接口
+ src/lrs：依赖 src/cc, src/link, src/sma 中的各种接口

其余部分可以通过阅读 README.md 获取
