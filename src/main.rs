mod circular_buf_mutex;
mod mpsc_unbounded;

fn main() {
    circular_buf_mutex::run_bench();
    // mpsc_unbounded::run_bench();
}
