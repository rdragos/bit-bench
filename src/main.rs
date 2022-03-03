use ndarray::prelude::*;
use structopt::StructOpt;
use bitvec::prelude::*;

#[derive(Debug, StructOpt, Clone)]
struct Opt {
    #[structopt(env, long, default_value="0")]
    exp_type: u16,

    #[structopt(env, long, default_value="1000")]
    n_exp: usize,
}

#[derive(Clone, PartialEq)]
pub struct BitArrayRepr {
    pub data: BitVec<u8, Lsb0>,
    pub dim: IxDyn,
}

impl BitArrayRepr {
    pub fn from_raw(data: BitVec<u8, Lsb0>, dim: IxDyn) -> Self {
        BitArrayRepr {
            data: data,
            dim,
        }
    }
}


fn main() {
    let opt = Opt::from_args();

    println!("benching with type = {:?}", opt.exp_type);
    if opt.exp_type == 128 {
        let x_backing: ArrayD<u128> = array![0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0]
            .into_dimensionality::<IxDyn>()
            .unwrap();
        let _duplicates: Vec<_> = (0..opt.n_exp).map(|_| x_backing.clone()).collect();
    }  else if opt.exp_type == 64 {
        let x_backing: ArrayD<u64> = array![0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0]
            .into_dimensionality::<IxDyn>()
            .unwrap();
        let _duplicates: Vec<_> = (0..opt.n_exp).map(|_| x_backing.clone()).collect();
    } else if opt.exp_type == 8 {
        let x_backing: ArrayD<u8> = array![0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0]
            .into_dimensionality::<IxDyn>()
            .unwrap();
        let _duplicates: Vec<_> = (0..opt.n_exp).map(|_| x_backing.clone()).collect();
    }
    else if opt.exp_type == 1 {
        let raw = array![0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0].into_dyn();
        let data = raw.as_slice().unwrap().iter().map(|&ai| ai != 0).collect();
        let bit_array = BitArrayRepr::from_raw(data, raw.dim());

        let _duplicates: Vec<_> = (0..opt.n_exp).map(|_| bit_array.clone()).collect();
    }
    else {
        println!("no option found");
    }
}
