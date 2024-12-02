let delta0 = head.0 - tail.0;
let delta1 = head.1 - tail.1;
if delta0.abs() > 1 || delta1.abs() > 1 {
    tail.0 += delta0.signum();
    tail.1 += delta1.signum();
}
