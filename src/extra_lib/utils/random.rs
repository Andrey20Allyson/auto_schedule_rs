use rand::Rng;

pub fn randomize_vec<T>(mut vec: Vec<T>) -> Vec<T> {
    let mut rand_gen = rand::thread_rng();
    let len = vec.len();

    for i in 0..len {
        let last_index = len - i - 1;
        let random_index = rand_gen.gen_range(0..=last_index);
        vec.swap(random_index, last_index);
    }

    vec
}

pub fn randomize_vec_ref<T>(vec: &mut Vec<T>) {
    let mut rand_gen = rand::thread_rng();
    let len = vec.len();

    for i in 0..len {
        let last_index = len - i - 1;
        let random_index = rand_gen.gen_range(0..=last_index);
        vec.swap(random_index, last_index);
    }
}

pub trait Randomizable<T> {
    fn rand(&self) -> T;
}

impl<T: Clone> Randomizable<Vec<T>> for Vec<T> {
    fn rand(&self) -> Vec<T> {
        randomize_vec(self.clone().to_vec())
    }
}
