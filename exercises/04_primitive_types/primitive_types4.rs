fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // Technically this is pointer to the array
        // and rust auto-dereferences to get the value of the array
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
