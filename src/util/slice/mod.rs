use std::ops;

pub fn range<R>(range: R, bounds: ops::RangeTo<usize>) -> ops::Range<usize>
where
    R: ops::RangeBounds<usize>,
{
    let len = bounds.end;

    let start: ops::Bound<&usize> = range.start_bound();
    let start = match start {
        ops::Bound::Included(&start) => start,
        ops::Bound::Excluded(start) => {
            start.checked_add(1).unwrap_or_else(|| panic!("Overflow on range start bound"))
        }
        ops::Bound::Unbounded => 0,
    };

    let end: ops::Bound<&usize> = range.end_bound();
    let end = match end {
        ops::Bound::Included(end) => {
            end.checked_add(1).unwrap_or_else(|| panic!("Overflow on range end bound"))
        }
        ops::Bound::Excluded(&end) => end,
        ops::Bound::Unbounded => len,
    };

    if start > end {
        panic!("Start index {} greater than end index {}", start, end);
    }
    if end > len {
        panic!("End index {} greater than length {}", end, len);
    }

    ops::Range { start, end }
}
