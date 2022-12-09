use ark_ff::fields::{Fp768, MontBackend};

pub type Fr = Fp768<MontBackend<FrConfig, 12>>;
#[derive(ark_ff::MontConfig)]
#[modulus = "41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888458477323173057491593855069696241854796396165721416325350064441470418137846398469611935719059908164220784476160001"]
#[generator = "17"]
#[small_subgroup_base = "5"]
#[small_subgroup_power = "2"]
pub struct FrConfig;

pub const FR_ONE: Fr = ark_ff::MontFp!("1");