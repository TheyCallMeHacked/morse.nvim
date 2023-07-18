use nvim_oxi as oxi;

#[oxi::module]
fn morse() -> oxi::Result<i32> {
    Ok(42)
}
