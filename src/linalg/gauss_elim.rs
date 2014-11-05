// std imports

// srmat imports

use matrix::MatrixF64;


/// A Gauss elimination problem specification
pub struct GaussElimination<'a, 'b>{
    pub a : &'a MatrixF64,
    pub b : &'b MatrixF64
}

impl<'a, 'b> GaussElimination<'a, 'b>{

    /// Setup of a new Gauss elimination problem.
    pub fn new(a : &'a MatrixF64, b : &'b MatrixF64) -> GaussElimination<'a, 'b>{
        assert!(a.is_square());
        assert_eq!(a.num_rows(), b.num_rows());
        GaussElimination{a : a , b : b}
    } 

    /// Carries out the procedure of Gauss elimination.
    pub fn solve(&self) -> MatrixF64 {
        let mut m = self.a.clone();
        m.append_columns(self.b);
        let rows = m.num_rows();
        let cols = m.num_cols();
        // Forward elimination process.
        for k in range(0, rows){
            // We are working on k-th column.
            // Create a view of the remaining elements in column
            //let col_k_remaining = m.view(k, k, rows - k, 1);
            //let (max_val, rr, _) = col_k_remaining.abs_max_scalar();
            //if rr > k {
            //    // We need to exchange rows of the submatrix.
            //}
            // Pick up the pivot
            let pivot = m.get(k, k);
            let mut lower_right  = m.view(k + 1, k, rows - k - 1, cols -k);
            //println!("Pivot: {}", pivot);
            //println!("{}", lower_right);
            for r in range(0, lower_right.num_rows()){
                let first = lower_right.get(r, 0);
                let factor = first  / pivot;
                lower_right.ero_scale_add(r, -1, -factor);
            }
            //println!("m: {}", m);
        }
        // Backward substitution starts now.
        let mut b = m.view(0, self.a.num_cols(), 
            self.b.num_rows(), 
            self.b.num_cols());
        let mut r = m.num_rows() - 1;
        loop {
            let pivot = m.get(r, r);
            b.ero_scale(r, 1.0/pivot);
            for j in range(r+1, m.num_rows()){
                let factor = m.get(r, j) / pivot;
                b.ero_scale_add(r, j as int, -factor);  
            }
            if r == 0 {
                break;
            }
            r -= 1;
        }
        // We extract the result.
        b.to_matrix()
    }
    
}