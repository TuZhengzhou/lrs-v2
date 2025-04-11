use ark_ec::{pairing::Pairing, CurveGroup};
use ark_ff::Zero;
use std::ops::{AddAssign, Mul};

///! This module contains the implementation of a sparse matrix
/// The sparse matrix is represented as a vector of columns
/// The columns are represented as a vector of CoeffPos
/// The CoeffPos struct contains the value and the position of the coefficient

///! Use column rather than row major representation is becuase in subspace snark, we always compute Mtx^T \cdot v, where Mtx is a sparse matrix and v is a vector
/// 
/// This file provides following functionality:
/// 
/// SparseMatrix::new(nr, nc) 
///     - create a new sparse matrix with nr rows and nc columns
/// 
/// SparseMatrix::insert_val(r, c, v) 
///     - insert a value v at row r and column c
/// 
/// SparseMatrix::insert_row_slice(r, c_offset, vs) 
///     - insert a contiguous sequence of values at row r starting from c_offset
/// 
/// SparseLinAlgebra::sparse_inner_product(v, w) 
///     - compute the inner product of a column of a sparse matrix and another (sparse) vector
/// 
/// SparseLinAlgebra::sparse_vector_matrix_mult(v, m, c) 
///     - compute the product of a sparse vector and a sparse matrix
/// 
/// inner_product(v, w)
///    - compute the inner product of two vectors
/// 
/// scalar_vector_mult(a, v, r)
///   - compute the product of a scalar and a vector

/// CoeffPos: A struct to help build sparse matrices.
#[derive(Clone, Debug)]
pub struct CoeffPos<T> {
    val: T,
    pos: usize,
}

// a column is a vector of CoeffPos-s
type Col<T> = Vec<CoeffPos<T>>;

/* TODO: One could consider a cache-ScalarFieldiendlier implementation for the 2-row case*/

/// Column-Major Sparse Matrix
#[derive(Clone, Debug)]
pub struct SparseMatrix<T> {
    cols: Vec<Col<T>>, // a vector of columns
    pub nr: usize,
    pub nc: usize,
}

impl<T: Copy> SparseMatrix<T> {
    // NB: Given column by column
    pub fn new(nr: usize, nc: usize) -> SparseMatrix<T> {
        SparseMatrix {
            cols: vec![vec![]; nc],
            nr,
            nc,
        }
    }

    pub fn insert_val(&mut self, r: usize, c: usize, v: &T) {
        let coeff_pos = CoeffPos { pos: r, val: *v };
        self.cols[c].push(coeff_pos);
    }

    // insert a continguous sequence of values at row r starting ScalarFieldom c_offset
    pub fn insert_row_slice(&mut self, r: usize, c_offset: usize, vs: Vec<T>) {
        // NB: could be improved in efficiency by first extending the vector
        for (i, x) in vs.iter().enumerate() {
            self.insert_val(r, c_offset + i, x);
        }
    }

    pub fn get_col(&self, c: usize) -> &Col<T> {
        &self.cols[c]
    }
}

pub struct SparseLinAlgebra<PE: Pairing> {
    pairing_engine_type: ark_std::marker::PhantomData<PE>,
}

impl<PE: Pairing> SparseLinAlgebra<PE> {
    // Inner product of a column of a sparse matrix and another (sparse) vector
    // this is basically a multi-exp
    pub fn sparse_inner_product(v: &Vec<PE::ScalarField>, w: &Col<PE::G1Affine>) -> PE::G1Affine {
        let mut res: PE::G1 = PE::G1::zero();
        for coeffpos in w {
            let g = coeffpos.val;
            let i = coeffpos.pos;
            // XXX: Should this be optimized for special cases
            //         (e.g. 0 or 1) or is this already in .mul?
            let tmp = g.mul(v[i]);

            res.add_assign(&tmp);
        }
        res.into_affine()
    }

    pub fn sparse_vector_matrix_mult(
        v: &Vec<PE::ScalarField>,
        m: &SparseMatrix<PE::G1Affine>,
        nc: usize,
    ) -> Vec<PE::G1Affine> {
        // the result should contain every column of m multiplied by v
        assert!(nc == m.nc);
        assert!(v.len() == m.nr);
        let mut res: Vec<PE::G1Affine> = Vec::with_capacity(nc);
        for c in 0..m.nc {
            res.push(Self::sparse_inner_product(&v, &m.get_col(c)));
        }
        res
    }
}

pub fn inner_product<PE: Pairing>(v: &[PE::ScalarField], w: &[PE::G1Affine]) -> PE::G1Affine {
    assert_eq!(v.len(), w.len());
    let mut res: PE::G1 = PE::G1::zero();
    for i in 0..v.len() {
        let tmp = w[i].mul(v[i]);
        res.add_assign(&tmp);
    }
    res.into_affine()
}

pub fn scalar_vector_mult<PE: Pairing>(a: &PE::ScalarField, v: &[PE::ScalarField], r: usize) -> Vec<PE::ScalarField> {
    let mut res: Vec<PE::ScalarField> = Vec::with_capacity(r);
    for i in 0..v.len() {
        let x: PE::ScalarField = a.mul(&v[i]);
        res.push(x);
    }
    res
}
