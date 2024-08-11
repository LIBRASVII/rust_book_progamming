fn main() {
    let array = [2, 4, 1, 5, 3];
    let iter = array.iter();
    let j = 2;
    let i = 1;

    for j in iter {
        let key = array[j];

        i = j - 1;

        while i > 0 && array[i] > key {
            array[i + 1] = array[i];
            i = i - 1;
        }
        array[i + 1] = key;
    }
}
