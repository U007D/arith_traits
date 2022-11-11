// TODO: Determine if [[i|u]8; 32] is a more performant way to represent the expanded working type
//       (i.e. `[i|u256]` proxy) for `[i|u]128` instead of `big_[u]int`, performance-wise.
mod big_int;
mod big_uint;
