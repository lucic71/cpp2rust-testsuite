extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type YGAlign = u32;
pub const YGAlign_YGAlignAuto: YGAlign = 0;
pub const YGAlign_YGAlignFlexStart: YGAlign = 1;
pub const YGAlign_YGAlignCenter: YGAlign = 2;
pub const YGAlign_YGAlignFlexEnd: YGAlign = 3;
pub const YGAlign_YGAlignStretch: YGAlign = 4;
pub const YGAlign_YGAlignBaseline: YGAlign = 5;
pub const YGAlign_YGAlignSpaceBetween: YGAlign = 6;
pub const YGAlign_YGAlignSpaceAround: YGAlign = 7;
pub const YGAlign_YGAlignSpaceEvenly: YGAlign = 8;
pub const YGAlign_YGAlignStart: YGAlign = 9;
pub const YGAlign_YGAlignEnd: YGAlign = 10;
pub type YGBoxSizing = u32;
pub const YGBoxSizing_YGBoxSizingBorderBox: YGBoxSizing = 0;
pub const YGBoxSizing_YGBoxSizingContentBox: YGBoxSizing = 1;
pub type YGDimension = u32;
pub const YGDimension_YGDimensionWidth: YGDimension = 0;
pub const YGDimension_YGDimensionHeight: YGDimension = 1;
pub type YGDirection = u32;
pub const YGDirection_YGDirectionInherit: YGDirection = 0;
pub const YGDirection_YGDirectionLTR: YGDirection = 1;
pub const YGDirection_YGDirectionRTL: YGDirection = 2;
pub type YGDisplay = u32;
pub const YGDisplay_YGDisplayFlex: YGDisplay = 0;
pub const YGDisplay_YGDisplayNone: YGDisplay = 1;
pub const YGDisplay_YGDisplayContents: YGDisplay = 2;
pub const YGDisplay_YGDisplayGrid: YGDisplay = 3;
pub type YGEdge = u32;
pub const YGEdge_YGEdgeLeft: YGEdge = 0;
pub const YGEdge_YGEdgeTop: YGEdge = 1;
pub const YGEdge_YGEdgeRight: YGEdge = 2;
pub const YGEdge_YGEdgeBottom: YGEdge = 3;
pub const YGEdge_YGEdgeStart: YGEdge = 4;
pub const YGEdge_YGEdgeEnd: YGEdge = 5;
pub const YGEdge_YGEdgeHorizontal: YGEdge = 6;
pub const YGEdge_YGEdgeVertical: YGEdge = 7;
pub const YGEdge_YGEdgeAll: YGEdge = 8;
pub type YGErrata = u32;
pub const YGErrata_YGErrataNone: YGErrata = 0;
pub const YGErrata_YGErrataStretchFlexBasis: YGErrata = 1;
pub const YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding: YGErrata = 2;
pub const YGErrata_YGErrataAbsolutePercentAgainstInnerSize: YGErrata = 4;
pub const YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto: YGErrata = 8;
pub const YGErrata_YGErrataFlexFirstPassUsesRunningTotals: YGErrata = 16;
pub const YGErrata_YGErrataAll: YGErrata = 2147483647;
pub const YGErrata_YGErrataClassic: YGErrata = 2147483646;
pub type YGExperimentalFeature = u32;
pub const YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis: YGExperimentalFeature = 0;
pub const YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent: YGExperimentalFeature =
    1;
pub type YGFlexDirection = u32;
pub const YGFlexDirection_YGFlexDirectionColumn: YGFlexDirection = 0;
pub const YGFlexDirection_YGFlexDirectionColumnReverse: YGFlexDirection = 1;
pub const YGFlexDirection_YGFlexDirectionRow: YGFlexDirection = 2;
pub const YGFlexDirection_YGFlexDirectionRowReverse: YGFlexDirection = 3;
pub type YGGridTrackType = u32;
pub const YGGridTrackType_YGGridTrackTypeAuto: YGGridTrackType = 0;
pub const YGGridTrackType_YGGridTrackTypePoints: YGGridTrackType = 1;
pub const YGGridTrackType_YGGridTrackTypePercent: YGGridTrackType = 2;
pub const YGGridTrackType_YGGridTrackTypeFr: YGGridTrackType = 3;
pub const YGGridTrackType_YGGridTrackTypeMinmax: YGGridTrackType = 4;
pub type YGGutter = u32;
pub const YGGutter_YGGutterColumn: YGGutter = 0;
pub const YGGutter_YGGutterRow: YGGutter = 1;
pub const YGGutter_YGGutterAll: YGGutter = 2;
pub type YGJustify = u32;
pub const YGJustify_YGJustifyAuto: YGJustify = 0;
pub const YGJustify_YGJustifyFlexStart: YGJustify = 1;
pub const YGJustify_YGJustifyCenter: YGJustify = 2;
pub const YGJustify_YGJustifyFlexEnd: YGJustify = 3;
pub const YGJustify_YGJustifySpaceBetween: YGJustify = 4;
pub const YGJustify_YGJustifySpaceAround: YGJustify = 5;
pub const YGJustify_YGJustifySpaceEvenly: YGJustify = 6;
pub const YGJustify_YGJustifyStretch: YGJustify = 7;
pub const YGJustify_YGJustifyStart: YGJustify = 8;
pub const YGJustify_YGJustifyEnd: YGJustify = 9;
pub type YGLogLevel = u32;
pub const YGLogLevel_YGLogLevelError: YGLogLevel = 0;
pub const YGLogLevel_YGLogLevelWarn: YGLogLevel = 1;
pub const YGLogLevel_YGLogLevelInfo: YGLogLevel = 2;
pub const YGLogLevel_YGLogLevelDebug: YGLogLevel = 3;
pub const YGLogLevel_YGLogLevelVerbose: YGLogLevel = 4;
pub const YGLogLevel_YGLogLevelFatal: YGLogLevel = 5;
pub type YGMeasureMode = u32;
pub const YGMeasureMode_YGMeasureModeUndefined: YGMeasureMode = 0;
pub const YGMeasureMode_YGMeasureModeExactly: YGMeasureMode = 1;
pub const YGMeasureMode_YGMeasureModeAtMost: YGMeasureMode = 2;
pub type YGNodeType = u32;
pub const YGNodeType_YGNodeTypeDefault: YGNodeType = 0;
pub const YGNodeType_YGNodeTypeText: YGNodeType = 1;
pub type YGOverflow = u32;
pub const YGOverflow_YGOverflowVisible: YGOverflow = 0;
pub const YGOverflow_YGOverflowHidden: YGOverflow = 1;
pub const YGOverflow_YGOverflowScroll: YGOverflow = 2;
pub type YGPositionType = u32;
pub const YGPositionType_YGPositionTypeStatic: YGPositionType = 0;
pub const YGPositionType_YGPositionTypeRelative: YGPositionType = 1;
pub const YGPositionType_YGPositionTypeAbsolute: YGPositionType = 2;
pub type YGUnit = u32;
pub const YGUnit_YGUnitUndefined: YGUnit = 0;
pub const YGUnit_YGUnitPoint: YGUnit = 1;
pub const YGUnit_YGUnitPercent: YGUnit = 2;
pub const YGUnit_YGUnitAuto: YGUnit = 3;
pub const YGUnit_YGUnitMaxContent: YGUnit = 4;
pub const YGUnit_YGUnitFitContent: YGUnit = 5;
pub const YGUnit_YGUnitStretch: YGUnit = 6;
pub type YGWrap = u32;
pub const YGWrap_YGWrapNoWrap: YGWrap = 0;
pub const YGWrap_YGWrapWrap: YGWrap = 1;
pub const YGWrap_YGWrapWrapReverse: YGWrap = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct YGSize {
    #[offset(0)]
    pub width: f32,
    #[offset(4)]
    pub height: f32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct YGValue {
    #[offset(0)]
    pub value: f32,
    #[offset(4)]
    pub unit: YGUnit,
}
pub fn __compareDoubles_3(mut a: AnyPtr, mut b: AnyPtr) -> i32 {
    let mut arg1: f64 = (a.reinterpret_cast::<f64>().read());
    let mut arg2: f64 = (b.reinterpret_cast::<f64>().read());
    if (((arg1 < arg2) as i32) != 0) {
        return -1_i32;
    }
    if (((arg1 > arg2) as i32) != 0) {
        return 1;
    }
    return 0;
}
pub fn __printBenchmarkResult_4(
    mut name: Ptr<i8>,
    mut start: ::libc::clock_t,
    mut endTimes: Ptr<::libc::clock_t>,
) {
    let timesInMs: Value<Box<[f64]>> = Rc::new(RefCell::new(
        (0..1000).map(|_| 0_f64).collect::<Box<[f64]>>(),
    ));
    let mut mean: f64 = 0_f64;
    let mut lastEnd: ::libc::clock_t = start;
    let mut i: u32 = 0_u32;
    'loop_: while (((i < 1000_u32) as i32) != 0) {
        (*timesInMs.borrow_mut())[(i) as usize] = {
            (((({ (elem!(endTimes, i).read()) } - { lastEnd }) as f64) / ((1000000 as i64) as f64))
                * 1000_f64)
        };
        lastEnd = { (elem!(endTimes, i).read()) };
        mean += (*timesInMs.borrow())[(i) as usize];
        i.postfix_inc();
    }
    mean /= 1000_f64;
    {
        let __base = ((timesInMs.as_pointer() as Ptr<f64>) as Ptr<f64>)
            .to_any()
            .reinterpret_cast::<u8>();
        for __i in 0..1000_usize {
            let mut __min = __i;
            for __j in (__i + 1)..1000_usize {
                if __compareDoubles_3(
                    __base.offset(__j * ::std::mem::size_of::<f64>()).to_any(),
                    __base.offset(__min * ::std::mem::size_of::<f64>()).to_any(),
                ) < 0
                {
                    __min = __j;
                }
            }
            if __min != __i {
                for __b in 0..::std::mem::size_of::<f64>() {
                    let __x = __base
                        .offset(__i * ::std::mem::size_of::<f64>() + __b)
                        .read();
                    let __y = __base
                        .offset(__min * ::std::mem::size_of::<f64>() + __b)
                        .read();
                    elem!(__base, __i * ::std::mem::size_of::<f64>() + __b).write(__y);
                    elem!(__base, __min * ::std::mem::size_of::<f64>() + __b).write(__x);
                }
            }
        }
    };
    let mut median: f64 = (*timesInMs.borrow())[(1000 / 2) as usize];
    let mut variance: f64 = 0_f64;
    let mut i: u32 = 0_u32;
    'loop_: while (((i < 1000_u32) as i32) != 0) {
        variance += ((*timesInMs.borrow())[(i) as usize] - mean).powf(2_f64);
        i.postfix_inc();
    }
    variance /= 1000_f64;
    let mut stddev: f64 = variance.sqrt();
    println!(
        "{}: median: {:.6} ms, stddev: {:.6} ms",
        name, median, stddev
    );
}
pub fn _measure_5(
    mut node: Ptr<YGNode>,
    mut width: f32,
    mut widthMode: YGMeasureMode,
    mut height: f32,
    mut heightMode: YGMeasureMode,
) -> YGSize {
    &(node);
    return YGSize {
        width: if ((((widthMode as u32) == ((YGMeasureMode_YGMeasureModeUndefined as i32) as u32))
            as i32)
            != 0)
        {
            10_f32
        } else {
            width
        },
        height: if ((((heightMode as u32) == ((YGMeasureMode_YGMeasureModeUndefined as i32) as u32))
            as i32)
            != 0)
        {
            10_f32
        } else {
            height
        },
    };
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<i8>>) -> i32 {
    &(argc);
    &(argv);
    let mut __start: ::libc::clock_t = 0 as ::libc::clock_t;
    let __endTimes: Value<Box<[::libc::clock_t]>> = Rc::new(RefCell::new(
        (0..1000)
            .map(|_| 0 as ::libc::clock_t)
            .collect::<Box<[::libc::clock_t]>>(),
    ));
    {
        {
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while (((__i < 1000_u32) as i32) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_6() });
                        ({ YGNodeStyleSetWidth_7((root).clone(), 100_f32) });
                        ({ YGNodeStyleSetHeight_8((root).clone(), 100_f32) });
                        let mut i: u32 = 0_u32;
                        'loop_: while (((i < 10_u32) as i32) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_6() });
                            ({
                                YGNodeSetMeasureFunc_9(
                                    (child).clone(),
                                    FnPtr::<
                                        fn(
                                            Ptr<YGNode>,
                                            f32,
                                            YGMeasureMode,
                                            f32,
                                            YGMeasureMode,
                                        ) -> YGSize,
                                    >::new(_measure_5),
                                )
                            });
                            ({ YGNodeStyleSetFlex_10((child).clone(), 1_f32) });
                            ({ YGNodeInsertChild_11((root).clone(), (child).clone(), 0_usize) });
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_13((root).clone()) });
                    }
                }
                let __rhs = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<i8>::from_string_literal(b"Stack with flex"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<::libc::clock_t>),
                )
            });
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while (((__i < 1000_u32) as i32) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_6() });
                        let mut i: u32 = 0_u32;
                        'loop_: while (((i < 10_u32) as i32) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_6() });
                            ({ YGNodeStyleSetHeight_8((child).clone(), 20_f32) });
                            ({
                                YGNodeSetMeasureFunc_9(
                                    (child).clone(),
                                    FnPtr::<
                                        fn(
                                            Ptr<YGNode>,
                                            f32,
                                            YGMeasureMode,
                                            f32,
                                            YGMeasureMode,
                                        ) -> YGSize,
                                    >::new(_measure_5),
                                )
                            });
                            ({ YGNodeInsertChild_11((root).clone(), (child).clone(), 0_usize) });
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_13((root).clone()) });
                    }
                }
                let __rhs = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<i8>::from_string_literal(b"Align stretch in undefined axis"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<::libc::clock_t>),
                )
            });
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while (((__i < 1000_u32) as i32) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_6() });
                        let mut i: u32 = 0_u32;
                        'loop_: while (((i < 10_u32) as i32) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_6() });
                            ({ YGNodeStyleSetFlex_10((child).clone(), 1_f32) });
                            ({ YGNodeInsertChild_11((root).clone(), (child).clone(), 0_usize) });
                            let mut ii: u32 = 0_u32;
                            'loop_: while (((ii < 10_u32) as i32) != 0) {
                                let mut grandChild: Ptr<YGNode> = ({ YGNodeNew_6() });
                                ({
                                    YGNodeSetMeasureFunc_9(
                                        (grandChild).clone(),
                                        FnPtr::<
                                            fn(
                                                Ptr<YGNode>,
                                                f32,
                                                YGMeasureMode,
                                                f32,
                                                YGMeasureMode,
                                            )
                                                -> YGSize,
                                        >::new(_measure_5),
                                    )
                                });
                                ({ YGNodeStyleSetFlex_10((grandChild).clone(), 1_f32) });
                                ({
                                    YGNodeInsertChild_11(
                                        (child).clone(),
                                        (grandChild).clone(),
                                        0_usize,
                                    )
                                });
                                ii.postfix_inc();
                            }
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_13((root).clone()) });
                    }
                }
                let __rhs = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<i8>::from_string_literal(b"Nested flex"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<::libc::clock_t>),
                )
            });
            __start = {
                let mut __ts = libc::timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
            };
            let mut __i: u32 = 0_u32;
            'loop_: while (((__i < 1000_u32) as i32) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_6() });
                        let mut i: u32 = 0_u32;
                        'loop_: while (((i < 10_u32) as i32) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_6() });
                            ({ YGNodeStyleSetFlexGrow_14((child).clone(), 1_f32) });
                            ({ YGNodeStyleSetWidth_7((child).clone(), 10_f32) });
                            ({ YGNodeStyleSetHeight_8((child).clone(), 10_f32) });
                            ({ YGNodeInsertChild_11((root).clone(), (child).clone(), 0_usize) });
                            let mut ii: u32 = 0_u32;
                            'loop_: while (((ii < 10_u32) as i32) != 0) {
                                let mut grandChild: Ptr<YGNode> = ({ YGNodeNew_6() });
                                ({
                                    YGNodeStyleSetFlexDirection_15(
                                        (grandChild).clone(),
                                        YGFlexDirection_YGFlexDirectionRow,
                                    )
                                });
                                ({ YGNodeStyleSetFlexGrow_14((grandChild).clone(), 1_f32) });
                                ({ YGNodeStyleSetWidth_7((grandChild).clone(), 10_f32) });
                                ({ YGNodeStyleSetHeight_8((grandChild).clone(), 10_f32) });
                                ({
                                    YGNodeInsertChild_11(
                                        (child).clone(),
                                        (grandChild).clone(),
                                        0_usize,
                                    )
                                });
                                let mut iii: u32 = 0_u32;
                                'loop_: while (((iii < 10_u32) as i32) != 0) {
                                    let mut grandGrandChild: Ptr<YGNode> = ({ YGNodeNew_6() });
                                    ({
                                        YGNodeStyleSetFlexGrow_14((grandGrandChild).clone(), 1_f32)
                                    });
                                    ({ YGNodeStyleSetWidth_7((grandGrandChild).clone(), 10_f32) });
                                    ({ YGNodeStyleSetHeight_8((grandGrandChild).clone(), 10_f32) });
                                    ({
                                        YGNodeInsertChild_11(
                                            (grandChild).clone(),
                                            (grandGrandChild).clone(),
                                            0_usize,
                                        )
                                    });
                                    let mut iiii: u32 = 0_u32;
                                    'loop_: while (((iiii < 10_u32) as i32) != 0) {
                                        let mut grandGrandGrandChild: Ptr<YGNode> =
                                            ({ YGNodeNew_6() });
                                        ({
                                            YGNodeStyleSetFlexDirection_15(
                                                (grandGrandGrandChild).clone(),
                                                YGFlexDirection_YGFlexDirectionRow,
                                            )
                                        });
                                        ({
                                            YGNodeStyleSetFlexGrow_14(
                                                (grandGrandGrandChild).clone(),
                                                1_f32,
                                            )
                                        });
                                        ({
                                            YGNodeStyleSetWidth_7(
                                                (grandGrandGrandChild).clone(),
                                                10_f32,
                                            )
                                        });
                                        ({
                                            YGNodeStyleSetHeight_8(
                                                (grandGrandGrandChild).clone(),
                                                10_f32,
                                            )
                                        });
                                        ({
                                            YGNodeInsertChild_11(
                                                (grandGrandChild).clone(),
                                                (grandGrandGrandChild).clone(),
                                                0_usize,
                                            )
                                        });
                                        iiii.postfix_inc();
                                    }
                                    iii.postfix_inc();
                                }
                                ii.postfix_inc();
                            }
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: f32 = (f32::NAN);
                            let _availableHeight: f32 = (f32::NAN);
                            YGNodeCalculateLayout_12(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_13((root).clone()) });
                    }
                }
                let __rhs = {
                    let mut __ts = libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    };
                    unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut __ts) };
                    (__ts.tv_sec * 1_000_000 + __ts.tv_nsec / 1_000) as ::libc::clock_t
                };
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<i8>::from_string_literal(b"Huge nested layout"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<::libc::clock_t>),
                )
            });
        }
    }
    return 0;
}
pub fn operator_bitnot_16(mut a: YGErrata) -> YGErrata {
    return ((!(a as u32)) as YGErrata);
}
pub fn operator_bitor_17(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as u32) | (b as u32)) as YGErrata);
}
pub fn operator_bitand_18(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as u32) & (b as u32)) as YGErrata);
}
pub fn operator_bitxor_19(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as u32) ^ (b as u32)) as YGErrata);
}
pub fn operator_bitor_assign_20(a: Ptr<YGErrata>, mut b: YGErrata) -> Ptr<YGErrata> {
    return {
        {
            let __rhs = (b as u32);
            a.reinterpret_cast::<u32>()
                .with_mut(|__v| *__v = *__v | __rhs)
        };
        a.reinterpret_cast::<u32>()
    }
    .reinterpret_cast::<YGErrata>();
}
pub fn operator_bitand_assign_21(a: Ptr<YGErrata>, mut b: YGErrata) -> Ptr<YGErrata> {
    return {
        {
            let __rhs = (b as u32);
            a.reinterpret_cast::<u32>()
                .with_mut(|__v| *__v = *__v & __rhs)
        };
        a.reinterpret_cast::<u32>()
    }
    .reinterpret_cast::<YGErrata>();
}
pub fn operator_bitxor_assign_22(a: Ptr<YGErrata>, mut b: YGErrata) -> Ptr<YGErrata> {
    return {
        {
            let __rhs = (b as u32);
            a.reinterpret_cast::<u32>()
                .with_mut(|__v| *__v = *__v ^ __rhs)
        };
        a.reinterpret_cast::<u32>()
    }
    .reinterpret_cast::<YGErrata>();
}
thread_local!(
    pub static YGUndefined_23: Value<f32> = Rc::new(RefCell::new(f32::NAN));
);
pub fn operator_eq_24(lhs: Ptr<YGValue>, rhs: Ptr<YGValue>) -> bool {
    if ({ (lhs.with(|__s| __s.unit) as i32) } != { (rhs.with(|__s| __s.unit) as i32) }) {
        return false;
    }
    'switch: {
        match { (lhs.with(|__s| __s.unit) as i32) } {
            __v if __v == (YGUnit_YGUnitUndefined as i32)
                || __v == (YGUnit_YGUnitAuto as i32)
                || __v == (YGUnit_YGUnitFitContent as i32)
                || __v == (YGUnit_YGUnitMaxContent as i32)
                || __v == (YGUnit_YGUnitStretch as i32) =>
            {
                return true;
            }
            __v if __v == (YGUnit_YGUnitPoint as i32) || __v == (YGUnit_YGUnitPercent as i32) => {
                return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
            }
            _ => {
                return false;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn operator_neg_25(value: Ptr<YGValue>) -> YGValue {
    return YGValue {
        value: -value.with(|__s| __s.value),
        unit: value.with(|__s| __s.unit),
    };
}
pub fn bitCount_26() -> i32 {
    return ((({ ordinalCount_27() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_28() -> i32 {
    return ((({ ordinalCount_29() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_30() -> i32 {
    return ((({ ordinalCount_31() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_32() -> i32 {
    return ((({ ordinalCount_33() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_34() -> i32 {
    return ((({ ordinalCount_35() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_36() -> i32 {
    return ((({ ordinalCount_37() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_38() -> i32 {
    return ((({ ordinalCount_39() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_40() -> i32 {
    return ((({ ordinalCount_41() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_42() -> i32 {
    return ((({ ordinalCount_43() }) - 1) as u8).cpp_bit_width();
}
pub fn bitCount_44() -> i32 {
    return ((({ ordinalCount_45() }) - 1) as u8).cpp_bit_width();
}
pub fn to_underlying_46(e: facebook_yoga_Edge) -> u8 {
    let e: Value<facebook_yoga_Edge> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as u8);
}
pub fn to_underlying_47(e: facebook_yoga_Dimension) -> u8 {
    let e: Value<facebook_yoga_Dimension> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as u8);
}
pub fn to_underlying_48(e: facebook_yoga_PhysicalEdge) -> u32 {
    let e: Value<facebook_yoga_PhysicalEdge> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as u32);
}
pub fn to_underlying_49(e: facebook_yoga_Gutter) -> u8 {
    let e: Value<facebook_yoga_Gutter> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as u8);
}
pub type facebook_yoga_Errata = u32;
pub const facebook_yoga_Errata_None: facebook_yoga_Errata = 0;
pub const facebook_yoga_Errata_StretchFlexBasis: facebook_yoga_Errata = 1;
pub const facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding: facebook_yoga_Errata =
    2;
pub const facebook_yoga_Errata_AbsolutePercentAgainstInnerSize: facebook_yoga_Errata = 4;
pub const facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto: facebook_yoga_Errata = 8;
pub const facebook_yoga_Errata_FlexFirstPassUsesRunningTotals: facebook_yoga_Errata = 16;
pub const facebook_yoga_Errata_All: facebook_yoga_Errata = 2147483647;
pub const facebook_yoga_Errata_Classic: facebook_yoga_Errata = 2147483646;
pub fn operator_bitnot_50(mut a: facebook_yoga_Errata) -> facebook_yoga_Errata {
    return ((!(a as u32)) as facebook_yoga_Errata);
}
pub fn operator_bitor_51(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as u32) | (b as u32)) as facebook_yoga_Errata);
}
pub fn operator_bitand_52(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as u32) & (b as u32)) as facebook_yoga_Errata);
}
pub fn operator_bitxor_53(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as u32) ^ (b as u32)) as facebook_yoga_Errata);
}
pub fn operator_bitor_assign_54(
    a: Ptr<facebook_yoga_Errata>,
    mut b: facebook_yoga_Errata,
) -> Ptr<facebook_yoga_Errata> {
    return {
        {
            let __rhs = (b as u32);
            a.reinterpret_cast::<u32>()
                .with_mut(|__v| *__v = *__v | __rhs)
        };
        a.reinterpret_cast::<u32>()
    }
    .reinterpret_cast::<facebook_yoga_Errata>();
}
pub fn operator_bitand_assign_55(
    a: Ptr<facebook_yoga_Errata>,
    mut b: facebook_yoga_Errata,
) -> Ptr<facebook_yoga_Errata> {
    return {
        {
            let __rhs = (b as u32);
            a.reinterpret_cast::<u32>()
                .with_mut(|__v| *__v = *__v & __rhs)
        };
        a.reinterpret_cast::<u32>()
    }
    .reinterpret_cast::<facebook_yoga_Errata>();
}
pub fn operator_bitxor_assign_56(
    a: Ptr<facebook_yoga_Errata>,
    mut b: facebook_yoga_Errata,
) -> Ptr<facebook_yoga_Errata> {
    return {
        {
            let __rhs = (b as u32);
            a.reinterpret_cast::<u32>()
                .with_mut(|__v| *__v = *__v ^ __rhs)
        };
        a.reinterpret_cast::<u32>()
    }
    .reinterpret_cast::<facebook_yoga_Errata>();
}
pub fn scopedEnum_57(mut unscoped: YGErrata) -> facebook_yoga_Errata {
    return (unscoped as facebook_yoga_Errata);
}
pub fn unscopedEnum_58(mut scoped: facebook_yoga_Errata) -> YGErrata {
    return (scoped as YGErrata);
}
pub fn toString_59(mut e: facebook_yoga_Errata) -> Ptr<i8> {
    return ({ YGErrataToString_60(({ unscopedEnum_58(e) })) });
}
pub type facebook_yoga_ExperimentalFeature = u8;
pub const facebook_yoga_ExperimentalFeature_WebFlexBasis: facebook_yoga_ExperimentalFeature = 0;
pub const facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent:
    facebook_yoga_ExperimentalFeature = 1;
pub fn ordinalCount_61() -> i32 {
    return 2;
}
pub fn scopedEnum_62(mut unscoped: YGExperimentalFeature) -> facebook_yoga_ExperimentalFeature {
    return (unscoped as facebook_yoga_ExperimentalFeature);
}
pub fn unscopedEnum_63(mut scoped: facebook_yoga_ExperimentalFeature) -> YGExperimentalFeature {
    return (scoped as YGExperimentalFeature);
}
pub fn toString_64(mut e: facebook_yoga_ExperimentalFeature) -> Ptr<i8> {
    return ({ YGExperimentalFeatureToString_65(({ unscopedEnum_63(e) })) });
}
pub type facebook_yoga_LogLevel = u8;
pub const facebook_yoga_LogLevel_Error: facebook_yoga_LogLevel = 0;
pub const facebook_yoga_LogLevel_Warn: facebook_yoga_LogLevel = 1;
pub const facebook_yoga_LogLevel_Info: facebook_yoga_LogLevel = 2;
pub const facebook_yoga_LogLevel_Debug: facebook_yoga_LogLevel = 3;
pub const facebook_yoga_LogLevel_Verbose: facebook_yoga_LogLevel = 4;
pub const facebook_yoga_LogLevel_Fatal: facebook_yoga_LogLevel = 5;
pub fn ordinalCount_66() -> i32 {
    return 6;
}
pub fn scopedEnum_67(mut unscoped: YGLogLevel) -> facebook_yoga_LogLevel {
    return (unscoped as facebook_yoga_LogLevel);
}
pub fn unscopedEnum_68(mut scoped: facebook_yoga_LogLevel) -> YGLogLevel {
    return (scoped as YGLogLevel);
}
pub fn toString_69(mut e: facebook_yoga_LogLevel) -> Ptr<i8> {
    return ({ YGLogLevelToString_70(({ unscopedEnum_68(e) })) });
}
pub type YGConfig = facebook_yoga_Config;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct facebook_yoga_Config {
    #[offset(0)]
    #[byte_size(8)]
    cloneNodeCallback_: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, usize) -> Ptr<YGNode>>,
    #[offset(8)]
    #[byte_size(8)]
    logger_: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>,
    #[offset(16)]
    useWebDefaults_: bool,
    #[offset(20)]
    version_: u32,
    #[offset(24)]
    #[byte_size(8)]
    experimentalFeatures_: Vec<bool>,
    #[offset(32)]
    #[byte_size(4)]
    errata_: facebook_yoga_Errata,
    #[offset(36)]
    pointScaleFactor_: f32,
    #[offset(40)]
    #[byte_size(8)]
    context_: AnyPtr,
}
impl facebook_yoga_Config {
    pub fn new(
        mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>,
    ) -> Self {
        Self {
            cloneNodeCallback_: FnPtr::<fn(Ptr<YGNode>, Ptr<YGNode>, usize) -> Ptr<YGNode>>::null(),
            logger_: (logger).clone(),
            useWebDefaults_: false,
            version_: 0_u32,
            experimentalFeatures_: vec![false; 2],
            errata_: ({
                operator_bitor_51(
                    facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                    facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                )
            }),
            pointScaleFactor_: 1_f32,
            context_: AnyPtr::default(),
        }
    }
    pub fn getDefault() -> Ptr<facebook_yoga_Config> {
        return defaultConfig__71.with(|v| v.as_pointer());
    }
}
impl Default for facebook_yoga_Config {
    fn default() -> Self {
        {
            facebook_yoga_Config {
                cloneNodeCallback_:
                    FnPtr::<fn(Ptr<YGNode>, Ptr<YGNode>, usize) -> Ptr<YGNode>>::null(),
                logger_: FnPtr::<
                    fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32,
                >::null(),
                useWebDefaults_: false,
                version_: 0_u32,
                experimentalFeatures_: vec![false; 2],
                errata_: ({
                    operator_bitor_51(
                        facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                        facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                    )
                }),
                pointScaleFactor_: 1_f32,
                context_: AnyPtr::default(),
            }
        }
    }
}
thread_local!(
    static defaultConfig__71: Value<facebook_yoga_Config> =
        Rc::new(RefCell::new(facebook_yoga_Config::new({
            FnPtr::<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>::null()
        })));
);
pub fn resolveRef_72(mut ref_: Ptr<YGConfig>) -> Ptr<facebook_yoga_Config> {
    return ref_.reinterpret_cast::<facebook_yoga_Config>();
}
pub fn resolveRef_73(mut ref_: Ptr<YGConfig>) -> Ptr<facebook_yoga_Config> {
    return ref_.reinterpret_cast::<facebook_yoga_Config>();
}
pub type facebook_yoga_Display = u8;
pub const facebook_yoga_Display_Flex: facebook_yoga_Display = 0;
pub const facebook_yoga_Display_None: facebook_yoga_Display = 1;
pub const facebook_yoga_Display_Contents: facebook_yoga_Display = 2;
pub const facebook_yoga_Display_Grid: facebook_yoga_Display = 3;
pub fn ordinalCount_41() -> i32 {
    return 4;
}
pub fn scopedEnum_74(mut unscoped: YGDisplay) -> facebook_yoga_Display {
    return (unscoped as facebook_yoga_Display);
}
pub fn unscopedEnum_75(mut scoped: facebook_yoga_Display) -> YGDisplay {
    return (scoped as YGDisplay);
}
pub fn toString_76(mut e: facebook_yoga_Display) -> Ptr<i8> {
    return ({ YGDisplayToString_77(({ unscopedEnum_75(e) })) });
}
pub fn operator_ne_78(
    a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    b: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
) -> bool {
    return ({ a.with(|__s| __s.node_.clone()) } != { b.with(|__s| __s.node_.clone()) })
        || ({ a.with(|__s| __s.childIndex_) } != { b.with(|__s| __s.childIndex_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(24)]
pub struct facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    #[offset(0)]
    #[byte_size(8)]
    node_: Ptr<facebook_yoga_Node>,
    #[offset(8)]
    childIndex_: usize,
    #[offset(16)]
    #[byte_size(8)]
    backtrack_: Vec<CppPair<Ptr<facebook_yoga_Node>, u64>>,
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    pub fn new_1(mut node: Ptr<facebook_yoga_Node>, mut childIndex: usize) -> Self {
        Self {
            node_: (node).clone(),
            childIndex_: childIndex,
            backtrack_: Vec::new(),
        }
    }
    pub fn move_from(
        _a0: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    ) -> Self {
        Self {
            node_: { (*_a0.upgrade().deref()).node_.clone() },
            childIndex_: { (*_a0.upgrade().deref()).childIndex_ },
            backtrack_: field!(_a0, backtrack_).with_mut(std::mem::take),
        }
    }
}
impl Default for facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    fn default() -> Self {
        {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
                node_: Ptr::<facebook_yoga_Node>::null(),
                childIndex_: 0_usize,
                backtrack_: Default::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
    #[offset(0)]
    #[byte_size(8)]
    node_: Ptr<facebook_yoga_Node>,
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
    pub fn new(mut node: Ptr<facebook_yoga_Node>) -> Self {
        let __this: facebook_yoga_LayoutableChildren_facebook_yoga_Node_ = Self {
            node_: (node).clone(),
        };
        const _: () = assert!(true, "std::input_iterator<LayoutableChildren<T>::Iterator>");;
        const _: () = assert!(true, "std::is_base_of<Node, T>::value");;
        __this
    }
}
pub type facebook_yoga_Dimension = u8;
pub const facebook_yoga_Dimension_Width: facebook_yoga_Dimension = 0;
pub const facebook_yoga_Dimension_Height: facebook_yoga_Dimension = 1;
pub fn ordinalCount_79() -> i32 {
    return 2;
}
pub fn scopedEnum_80(mut unscoped: YGDimension) -> facebook_yoga_Dimension {
    return (unscoped as facebook_yoga_Dimension);
}
pub fn unscopedEnum_81(mut scoped: facebook_yoga_Dimension) -> YGDimension {
    return (scoped as YGDimension);
}
pub fn toString_82(mut e: facebook_yoga_Dimension) -> Ptr<i8> {
    return ({ YGDimensionToString_83(({ unscopedEnum_81(e) })) });
}
pub type facebook_yoga_Direction = u8;
pub const facebook_yoga_Direction_Inherit: facebook_yoga_Direction = 0;
pub const facebook_yoga_Direction_LTR: facebook_yoga_Direction = 1;
pub const facebook_yoga_Direction_RTL: facebook_yoga_Direction = 2;
pub fn ordinalCount_27() -> i32 {
    return 3;
}
pub fn scopedEnum_84(mut unscoped: YGDirection) -> facebook_yoga_Direction {
    return (unscoped as facebook_yoga_Direction);
}
pub fn unscopedEnum_85(mut scoped: facebook_yoga_Direction) -> YGDirection {
    return (scoped as YGDirection);
}
pub fn toString_86(mut e: facebook_yoga_Direction) -> Ptr<i8> {
    return ({ YGDirectionToString_87(({ unscopedEnum_85(e) })) });
}
pub type facebook_yoga_Edge = u8;
pub const facebook_yoga_Edge_Left: facebook_yoga_Edge = 0;
pub const facebook_yoga_Edge_Top: facebook_yoga_Edge = 1;
pub const facebook_yoga_Edge_Right: facebook_yoga_Edge = 2;
pub const facebook_yoga_Edge_Bottom: facebook_yoga_Edge = 3;
pub const facebook_yoga_Edge_Start: facebook_yoga_Edge = 4;
pub const facebook_yoga_Edge_End: facebook_yoga_Edge = 5;
pub const facebook_yoga_Edge_Horizontal: facebook_yoga_Edge = 6;
pub const facebook_yoga_Edge_Vertical: facebook_yoga_Edge = 7;
pub const facebook_yoga_Edge_All: facebook_yoga_Edge = 8;
pub fn ordinalCount_88() -> i32 {
    return 9;
}
pub fn scopedEnum_89(mut unscoped: YGEdge) -> facebook_yoga_Edge {
    return (unscoped as facebook_yoga_Edge);
}
pub fn unscopedEnum_90(mut scoped: facebook_yoga_Edge) -> YGEdge {
    return (scoped as YGEdge);
}
pub fn toString_91(mut e: facebook_yoga_Edge) -> Ptr<i8> {
    return ({ YGEdgeToString_92(({ unscopedEnum_90(e) })) });
}
pub type facebook_yoga_MeasureMode = u8;
pub const facebook_yoga_MeasureMode_Undefined: facebook_yoga_MeasureMode = 0;
pub const facebook_yoga_MeasureMode_Exactly: facebook_yoga_MeasureMode = 1;
pub const facebook_yoga_MeasureMode_AtMost: facebook_yoga_MeasureMode = 2;
pub fn ordinalCount_93() -> i32 {
    return 3;
}
pub fn scopedEnum_94(mut unscoped: YGMeasureMode) -> facebook_yoga_MeasureMode {
    return (unscoped as facebook_yoga_MeasureMode);
}
pub fn unscopedEnum_95(mut scoped: facebook_yoga_MeasureMode) -> YGMeasureMode {
    return (scoped as YGMeasureMode);
}
pub fn toString_96(mut e: facebook_yoga_MeasureMode) -> Ptr<i8> {
    return ({ YGMeasureModeToString_97(({ unscopedEnum_95(e) })) });
}
pub type facebook_yoga_NodeType = u8;
pub const facebook_yoga_NodeType_Default: facebook_yoga_NodeType = 0;
pub const facebook_yoga_NodeType_Text: facebook_yoga_NodeType = 1;
pub fn ordinalCount_45() -> i32 {
    return 2;
}
pub fn scopedEnum_98(mut unscoped: YGNodeType) -> facebook_yoga_NodeType {
    return (unscoped as facebook_yoga_NodeType);
}
pub fn unscopedEnum_99(mut scoped: facebook_yoga_NodeType) -> YGNodeType {
    return (scoped as YGNodeType);
}
pub fn toString_100(mut e: facebook_yoga_NodeType) -> Ptr<i8> {
    return ({ YGNodeTypeToString_101(({ unscopedEnum_99(e) })) });
}
pub type facebook_yoga_PhysicalEdge = u32;
pub const facebook_yoga_PhysicalEdge_Left: facebook_yoga_PhysicalEdge = 0;
pub const facebook_yoga_PhysicalEdge_Top: facebook_yoga_PhysicalEdge = 1;
pub const facebook_yoga_PhysicalEdge_Right: facebook_yoga_PhysicalEdge = 2;
pub const facebook_yoga_PhysicalEdge_Bottom: facebook_yoga_PhysicalEdge = 3;
pub type facebook_yoga_SizingMode = i32;
pub const facebook_yoga_SizingMode_StretchFit: facebook_yoga_SizingMode = 0;
pub const facebook_yoga_SizingMode_MaxContent: facebook_yoga_SizingMode = 1;
pub const facebook_yoga_SizingMode_FitContent: facebook_yoga_SizingMode = 2;
pub fn measureMode_102(mut mode: facebook_yoga_SizingMode) -> facebook_yoga_MeasureMode {
    switch!(match mode {
        __v if __v == facebook_yoga_SizingMode_StretchFit => {
            return facebook_yoga_MeasureMode_Exactly;
        }
        __v if __v == facebook_yoga_SizingMode_MaxContent => {
            return facebook_yoga_MeasureMode_Undefined;
        }
        __v if __v == facebook_yoga_SizingMode_FitContent => {
            return facebook_yoga_MeasureMode_AtMost;
        }
        _ => {
            ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid SizingMode")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn sizingMode_104(mut mode: facebook_yoga_MeasureMode) -> facebook_yoga_SizingMode {
    switch!(match mode {
        __v if __v == facebook_yoga_MeasureMode_Exactly => {
            return facebook_yoga_SizingMode_StretchFit;
        }
        __v if __v == facebook_yoga_MeasureMode_Undefined => {
            return facebook_yoga_SizingMode_MaxContent;
        }
        __v if __v == facebook_yoga_MeasureMode_AtMost => {
            return facebook_yoga_SizingMode_FitContent;
        }
        _ => {
            ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid MeasureMode")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn isUndefined_105(mut value: f32) -> bool {
    return (value != value);
}
pub fn isUndefined_106(mut value: f64) -> bool {
    return (value != value);
}
pub fn isDefined_107(mut value: f32) -> bool {
    return !({ isUndefined_105(value) });
}
pub fn isDefined_108(mut value: f64) -> bool {
    return !({ isUndefined_106(value) });
}
pub fn isinf_109(value: f32) -> bool {
    let value: Value<f32> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) == f32::INFINITY) || ((*value.borrow()) == -f32::INFINITY);
}
pub fn maxOrDefined_110(a: f32, b: f32) -> f32 {
    let a: Value<f32> = Rc::new(RefCell::new(a));
    let b: Value<f32> = Rc::new(RefCell::new(b));
    if ({ isDefined_107((*a.borrow())) }) && ({ isDefined_107((*b.borrow())) }) {
        return ({
            let __a = a.as_pointer();
            let __b = b.as_pointer();
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read());
    }
    return if ({ isUndefined_105((*a.borrow())) }) {
        (*b.borrow())
    } else {
        (*a.borrow())
    };
}
pub fn inexactEquals_111(mut a: f32, mut b: f32) -> bool {
    if ({ isDefined_107(a) }) && ({ isDefined_107(b) }) {
        return ((a - b).abs() < 1e-04_f32);
    }
    return ({ isUndefined_105(a) }) && ({ isUndefined_105(b) });
}
pub fn inexactEquals_112(mut a: f64, mut b: f64) -> bool {
    if ({ isDefined_108(a) }) && ({ isDefined_108(b) }) {
        return ((a - b).abs() < 1e-04_f64);
    }
    return ({ isUndefined_106(a) }) && ({ isUndefined_106(b) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct facebook_yoga_CachedMeasurement {
    #[offset(0)]
    pub availableWidth: f32,
    #[offset(4)]
    pub availableHeight: f32,
    #[offset(8)]
    #[byte_size(4)]
    pub widthSizingMode: facebook_yoga_SizingMode,
    #[offset(12)]
    #[byte_size(4)]
    pub heightSizingMode: facebook_yoga_SizingMode,
    #[offset(16)]
    pub computedWidth: f32,
    #[offset(20)]
    pub computedHeight: f32,
    #[offset(24)]
    pub ownerWidth: f32,
    #[offset(28)]
    pub ownerHeight: f32,
}
impl CppEq for facebook_yoga_CachedMeasurement {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_CachedMeasurementImpl::operator_eq(__lhs, __rhs.read())
    }
}
impl Default for facebook_yoga_CachedMeasurement {
    fn default() -> Self {
        {
            facebook_yoga_CachedMeasurement {
                availableWidth: (-1_i32 as f32),
                availableHeight: (-1_i32 as f32),
                widthSizingMode: facebook_yoga_SizingMode_MaxContent,
                heightSizingMode: facebook_yoga_SizingMode_MaxContent,
                computedWidth: (-1_i32 as f32),
                computedHeight: (-1_i32 as f32),
                ownerWidth: (-1_i32 as f32),
                ownerHeight: (-1_i32 as f32),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct facebook_yoga_FloatOptional {
    #[offset(0)]
    value_: f32,
}
impl facebook_yoga_FloatOptional {
    pub fn new_1(mut value: f32) -> Self {
        Self { value_: value }
    }
}
impl CppLess for facebook_yoga_FloatOptional {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_113(__lhs.read(), __rhs.read())
    }
}
impl CppEq for facebook_yoga_FloatOptional {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_114(__lhs.read(), __rhs.read())
    }
}
impl Default for facebook_yoga_FloatOptional {
    fn default() -> Self {
        { facebook_yoga_FloatOptional { value_: f32::NAN } }
    }
}
pub fn operator_eq_114(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> bool {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
        == ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }))
        || (({ facebook_yoga_FloatOptionalImpl::isUndefined(&lhs.as_pointer()) })
            && ({ facebook_yoga_FloatOptionalImpl::isUndefined(&rhs.as_pointer()) }));
}
pub fn operator_eq_115(mut lhs: facebook_yoga_FloatOptional, mut rhs: f32) -> bool {
    return ({
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_114(_lhs, facebook_yoga_FloatOptional::new_1({ rhs }))
    });
}
pub fn operator_eq_116(mut lhs: f32, mut rhs: facebook_yoga_FloatOptional) -> bool {
    return ({
        let _lhs: facebook_yoga_FloatOptional = rhs;
        operator_eq_115(_lhs, lhs)
    });
}
pub fn operator_add_117(
    lhs: facebook_yoga_FloatOptional,
    rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return facebook_yoga_FloatOptional::new_1({
        (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
            + ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }))
    });
}
pub fn operator_gt_118(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> bool {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
        > ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }));
}
pub fn operator_lt_113(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> bool {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
        < ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }));
}
pub fn operator_ge_119(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_gt_118(_lhs, (rhs).clone())
    }) || ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_eq_114(_lhs, (rhs).clone())
    });
}
pub fn operator_le_120(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> bool {
    return ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_lt_113(_lhs, (rhs).clone())
    }) || ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_eq_114(_lhs, (rhs).clone())
    });
}
pub fn maxOrDefined_121(
    lhs: facebook_yoga_FloatOptional,
    rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return facebook_yoga_FloatOptional::new_1({
        ({
            let _a: f32 = ({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) });
            let _b: f32 = ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) });
            maxOrDefined_110(_a, _b)
        })
    });
}
pub fn inexactEquals_122(
    lhs: facebook_yoga_FloatOptional,
    rhs: facebook_yoga_FloatOptional,
) -> bool {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return ({
        let _a: f32 = ({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) });
        let _b: f32 = ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) });
        inexactEquals_111(_a, _b)
    });
}
thread_local!(
    pub static MaxCachedMeasurements_123: Value<i32> = Rc::new(RefCell::new(8));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(432)]
pub struct facebook_yoga_LayoutResults {
    #[offset(0)]
    pub computedFlexBasisGeneration: u32,
    #[offset(4)]
    #[byte_size(4)]
    pub computedFlexBasis: facebook_yoga_FloatOptional,
    #[offset(8)]
    #[byte_size(4)]
    pub computedAutoMinMainSize: facebook_yoga_FloatOptional,
    #[offset(12)]
    #[byte_size(8)]
    pub minContentGeneration: Vec<u32>,
    #[offset(20)]
    #[byte_size(8)]
    pub minContentOwnerWidth: Vec<f32>,
    #[offset(28)]
    #[byte_size(8)]
    pub minContent: Vec<f32>,
    #[offset(36)]
    pub generationCount: u32,
    #[offset(40)]
    pub configVersion: u32,
    #[offset(44)]
    #[byte_size(1)]
    pub lastOwnerDirection: facebook_yoga_Direction,
    #[offset(48)]
    pub nextCachedMeasurementsIndex: u32,
    #[offset(52)]
    #[byte_size(256)]
    pub cachedMeasurements: Vec<facebook_yoga_CachedMeasurement>,
    #[offset(308)]
    #[byte_size(32)]
    pub cachedLayout: facebook_yoga_CachedMeasurement,
    #[offset(340)]
    #[byte_size(1)]
    direction_: facebook_yoga_Direction,
    #[offset(340)]
    hadOverflow_: bool,
    #[offset(344)]
    #[byte_size(8)]
    dimensions_: Vec<f32>,
    #[offset(352)]
    #[byte_size(8)]
    measuredDimensions_: Vec<f32>,
    #[offset(360)]
    #[byte_size(8)]
    rawDimensions_: Vec<f32>,
    #[offset(368)]
    #[byte_size(16)]
    position_: Vec<f32>,
    #[offset(384)]
    #[byte_size(16)]
    margin_: Vec<f32>,
    #[offset(400)]
    #[byte_size(16)]
    border_: Vec<f32>,
    #[offset(416)]
    #[byte_size(16)]
    padding_: Vec<f32>,
}
impl Default for facebook_yoga_LayoutResults {
    fn default() -> Self {
        {
            facebook_yoga_LayoutResults {
                computedFlexBasisGeneration: 0_u32,
                computedFlexBasis: <facebook_yoga_FloatOptional>::default(),
                computedAutoMinMainSize: <facebook_yoga_FloatOptional>::default(),
                minContentGeneration: std::array::from_fn::<_, 2, _>(|_| 0_u32).to_vec(),
                minContentOwnerWidth: Vec::from([
                    YGUndefined_23.with(|rc| *rc.borrow()),
                    YGUndefined_23.with(|rc| *rc.borrow()),
                ]),
                minContent: Vec::from([
                    YGUndefined_23.with(|rc| *rc.borrow()),
                    YGUndefined_23.with(|rc| *rc.borrow()),
                ]),
                generationCount: 0_u32,
                configVersion: 0_u32,
                lastOwnerDirection: facebook_yoga_Direction_Inherit,
                nextCachedMeasurementsIndex: 0_u32,
                cachedMeasurements: std::array::from_fn::<_, 8, _>(|_| {
                    <facebook_yoga_CachedMeasurement>::default()
                })
                .to_vec(),
                cachedLayout: <facebook_yoga_CachedMeasurement>::default(),
                direction_: facebook_yoga_Direction_Inherit,
                hadOverflow_: false,
                dimensions_: Vec::from([
                    YGUndefined_23.with(|rc| *rc.borrow()),
                    YGUndefined_23.with(|rc| *rc.borrow()),
                ]),
                measuredDimensions_: Vec::from([
                    YGUndefined_23.with(|rc| *rc.borrow()),
                    YGUndefined_23.with(|rc| *rc.borrow()),
                ]),
                rawDimensions_: Vec::from([
                    YGUndefined_23.with(|rc| *rc.borrow()),
                    YGUndefined_23.with(|rc| *rc.borrow()),
                ]),
                position_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
                margin_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
                border_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
                padding_: std::array::from_fn::<_, 4, _>(|_| 0_f32).to_vec(),
            }
        }
    }
}
pub type facebook_yoga_FlexDirection = u8;
pub const facebook_yoga_FlexDirection_Column: facebook_yoga_FlexDirection = 0;
pub const facebook_yoga_FlexDirection_ColumnReverse: facebook_yoga_FlexDirection = 1;
pub const facebook_yoga_FlexDirection_Row: facebook_yoga_FlexDirection = 2;
pub const facebook_yoga_FlexDirection_RowReverse: facebook_yoga_FlexDirection = 3;
pub fn ordinalCount_29() -> i32 {
    return 4;
}
pub fn scopedEnum_124(mut unscoped: YGFlexDirection) -> facebook_yoga_FlexDirection {
    return (unscoped as facebook_yoga_FlexDirection);
}
pub fn unscopedEnum_125(mut scoped: facebook_yoga_FlexDirection) -> YGFlexDirection {
    return (scoped as YGFlexDirection);
}
pub fn toString_126(mut e: facebook_yoga_FlexDirection) -> Ptr<i8> {
    return ({ YGFlexDirectionToString_127(({ unscopedEnum_125(e) })) });
}
pub fn isRow_128(mut flexDirection: facebook_yoga_FlexDirection) -> bool {
    return (flexDirection == facebook_yoga_FlexDirection_Row)
        || (flexDirection == facebook_yoga_FlexDirection_RowReverse);
}
pub fn isColumn_129(mut flexDirection: facebook_yoga_FlexDirection) -> bool {
    return (flexDirection == facebook_yoga_FlexDirection_Column)
        || (flexDirection == facebook_yoga_FlexDirection_ColumnReverse);
}
pub fn resolveDirection_130(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_FlexDirection {
    if (direction == facebook_yoga_Direction_RTL) {
        if (flexDirection == facebook_yoga_FlexDirection_Row) {
            return facebook_yoga_FlexDirection_RowReverse;
        } else if (flexDirection == facebook_yoga_FlexDirection_RowReverse) {
            return facebook_yoga_FlexDirection_Row;
        }
    }
    return flexDirection;
}
pub fn resolveCrossDirection_131(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_FlexDirection {
    return if ({ isColumn_129(flexDirection) }) {
        ({ resolveDirection_130(facebook_yoga_FlexDirection_Row, direction) })
    } else {
        facebook_yoga_FlexDirection_Column
    };
}
pub fn flexStartEdge_132(
    mut flexDirection: facebook_yoga_FlexDirection,
) -> facebook_yoga_PhysicalEdge {
    switch!(match flexDirection {
        __v if __v == facebook_yoga_FlexDirection_Column => {
            return facebook_yoga_PhysicalEdge_Top;
        }
        __v if __v == facebook_yoga_FlexDirection_ColumnReverse => {
            return facebook_yoga_PhysicalEdge_Bottom;
        }
        __v if __v == facebook_yoga_FlexDirection_Row => {
            return facebook_yoga_PhysicalEdge_Left;
        }
        __v if __v == facebook_yoga_FlexDirection_RowReverse => {
            return facebook_yoga_PhysicalEdge_Right;
        }
        _ => {
            ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid FlexDirection")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn flexEndEdge_133(
    mut flexDirection: facebook_yoga_FlexDirection,
) -> facebook_yoga_PhysicalEdge {
    switch!(match flexDirection {
        __v if __v == facebook_yoga_FlexDirection_Column => {
            return facebook_yoga_PhysicalEdge_Bottom;
        }
        __v if __v == facebook_yoga_FlexDirection_ColumnReverse => {
            return facebook_yoga_PhysicalEdge_Top;
        }
        __v if __v == facebook_yoga_FlexDirection_Row => {
            return facebook_yoga_PhysicalEdge_Right;
        }
        __v if __v == facebook_yoga_FlexDirection_RowReverse => {
            return facebook_yoga_PhysicalEdge_Left;
        }
        _ => {
            ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid FlexDirection")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn inlineStartEdge_134(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if ({ isRow_128(flexDirection) }) {
        return if (direction == facebook_yoga_Direction_RTL) {
            facebook_yoga_PhysicalEdge_Right
        } else {
            facebook_yoga_PhysicalEdge_Left
        };
    }
    return facebook_yoga_PhysicalEdge_Top;
}
pub fn inlineEndEdge_135(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if ({ isRow_128(flexDirection) }) {
        return if (direction == facebook_yoga_Direction_RTL) {
            facebook_yoga_PhysicalEdge_Left
        } else {
            facebook_yoga_PhysicalEdge_Right
        };
    }
    return facebook_yoga_PhysicalEdge_Bottom;
}
pub fn dimension_136(mut flexDirection: facebook_yoga_FlexDirection) -> facebook_yoga_Dimension {
    switch!(match flexDirection {
        __v if __v == facebook_yoga_FlexDirection_Column => {
            return facebook_yoga_Dimension_Height;
        }
        __v if __v == facebook_yoga_FlexDirection_ColumnReverse => {
            return facebook_yoga_Dimension_Height;
        }
        __v if __v == facebook_yoga_FlexDirection_Row => {
            return facebook_yoga_Dimension_Width;
        }
        __v if __v == facebook_yoga_FlexDirection_RowReverse => {
            return facebook_yoga_Dimension_Width;
        }
        _ => {
            ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid FlexDirection")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub type facebook_yoga_Align = u8;
pub const facebook_yoga_Align_Auto: facebook_yoga_Align = 0;
pub const facebook_yoga_Align_FlexStart: facebook_yoga_Align = 1;
pub const facebook_yoga_Align_Center: facebook_yoga_Align = 2;
pub const facebook_yoga_Align_FlexEnd: facebook_yoga_Align = 3;
pub const facebook_yoga_Align_Stretch: facebook_yoga_Align = 4;
pub const facebook_yoga_Align_Baseline: facebook_yoga_Align = 5;
pub const facebook_yoga_Align_SpaceBetween: facebook_yoga_Align = 6;
pub const facebook_yoga_Align_SpaceAround: facebook_yoga_Align = 7;
pub const facebook_yoga_Align_SpaceEvenly: facebook_yoga_Align = 8;
pub const facebook_yoga_Align_Start: facebook_yoga_Align = 9;
pub const facebook_yoga_Align_End: facebook_yoga_Align = 10;
pub fn ordinalCount_33() -> i32 {
    return 11;
}
pub fn scopedEnum_137(mut unscoped: YGAlign) -> facebook_yoga_Align {
    return (unscoped as facebook_yoga_Align);
}
pub fn unscopedEnum_138(mut scoped: facebook_yoga_Align) -> YGAlign {
    return (scoped as YGAlign);
}
pub fn toString_139(mut e: facebook_yoga_Align) -> Ptr<i8> {
    return ({ YGAlignToString_140(({ unscopedEnum_138(e) })) });
}
pub type facebook_yoga_BoxSizing = u8;
pub const facebook_yoga_BoxSizing_BorderBox: facebook_yoga_BoxSizing = 0;
pub const facebook_yoga_BoxSizing_ContentBox: facebook_yoga_BoxSizing = 1;
pub fn ordinalCount_43() -> i32 {
    return 2;
}
pub fn scopedEnum_141(mut unscoped: YGBoxSizing) -> facebook_yoga_BoxSizing {
    return (unscoped as facebook_yoga_BoxSizing);
}
pub fn unscopedEnum_142(mut scoped: facebook_yoga_BoxSizing) -> YGBoxSizing {
    return (scoped as YGBoxSizing);
}
pub fn toString_143(mut e: facebook_yoga_BoxSizing) -> Ptr<i8> {
    return ({ YGBoxSizingToString_144(({ unscopedEnum_142(e) })) });
}
pub type facebook_yoga_Gutter = u8;
pub const facebook_yoga_Gutter_Column: facebook_yoga_Gutter = 0;
pub const facebook_yoga_Gutter_Row: facebook_yoga_Gutter = 1;
pub const facebook_yoga_Gutter_All: facebook_yoga_Gutter = 2;
pub fn ordinalCount_145() -> i32 {
    return 3;
}
pub fn scopedEnum_146(mut unscoped: YGGutter) -> facebook_yoga_Gutter {
    return (unscoped as facebook_yoga_Gutter);
}
pub fn unscopedEnum_147(mut scoped: facebook_yoga_Gutter) -> YGGutter {
    return (scoped as YGGutter);
}
pub fn toString_148(mut e: facebook_yoga_Gutter) -> Ptr<i8> {
    return ({ YGGutterToString_149(({ unscopedEnum_147(e) })) });
}
pub type facebook_yoga_Justify = u8;
pub const facebook_yoga_Justify_Auto: facebook_yoga_Justify = 0;
pub const facebook_yoga_Justify_FlexStart: facebook_yoga_Justify = 1;
pub const facebook_yoga_Justify_Center: facebook_yoga_Justify = 2;
pub const facebook_yoga_Justify_FlexEnd: facebook_yoga_Justify = 3;
pub const facebook_yoga_Justify_SpaceBetween: facebook_yoga_Justify = 4;
pub const facebook_yoga_Justify_SpaceAround: facebook_yoga_Justify = 5;
pub const facebook_yoga_Justify_SpaceEvenly: facebook_yoga_Justify = 6;
pub const facebook_yoga_Justify_Stretch: facebook_yoga_Justify = 7;
pub const facebook_yoga_Justify_Start: facebook_yoga_Justify = 8;
pub const facebook_yoga_Justify_End: facebook_yoga_Justify = 9;
pub fn ordinalCount_31() -> i32 {
    return 10;
}
pub fn scopedEnum_150(mut unscoped: YGJustify) -> facebook_yoga_Justify {
    return (unscoped as facebook_yoga_Justify);
}
pub fn unscopedEnum_151(mut scoped: facebook_yoga_Justify) -> YGJustify {
    return (scoped as YGJustify);
}
pub fn toString_152(mut e: facebook_yoga_Justify) -> Ptr<i8> {
    return ({ YGJustifyToString_153(({ unscopedEnum_151(e) })) });
}
pub type facebook_yoga_Overflow = u8;
pub const facebook_yoga_Overflow_Visible: facebook_yoga_Overflow = 0;
pub const facebook_yoga_Overflow_Hidden: facebook_yoga_Overflow = 1;
pub const facebook_yoga_Overflow_Scroll: facebook_yoga_Overflow = 2;
pub fn ordinalCount_39() -> i32 {
    return 3;
}
pub fn scopedEnum_154(mut unscoped: YGOverflow) -> facebook_yoga_Overflow {
    return (unscoped as facebook_yoga_Overflow);
}
pub fn unscopedEnum_155(mut scoped: facebook_yoga_Overflow) -> YGOverflow {
    return (scoped as YGOverflow);
}
pub fn toString_156(mut e: facebook_yoga_Overflow) -> Ptr<i8> {
    return ({ YGOverflowToString_157(({ unscopedEnum_155(e) })) });
}
pub type facebook_yoga_PositionType = u8;
pub const facebook_yoga_PositionType_Static: facebook_yoga_PositionType = 0;
pub const facebook_yoga_PositionType_Relative: facebook_yoga_PositionType = 1;
pub const facebook_yoga_PositionType_Absolute: facebook_yoga_PositionType = 2;
pub fn ordinalCount_35() -> i32 {
    return 3;
}
pub fn scopedEnum_158(mut unscoped: YGPositionType) -> facebook_yoga_PositionType {
    return (unscoped as facebook_yoga_PositionType);
}
pub fn unscopedEnum_159(mut scoped: facebook_yoga_PositionType) -> YGPositionType {
    return (scoped as YGPositionType);
}
pub fn toString_160(mut e: facebook_yoga_PositionType) -> Ptr<i8> {
    return ({ YGPositionTypeToString_161(({ unscopedEnum_159(e) })) });
}
pub type facebook_yoga_Unit = u8;
pub const facebook_yoga_Unit_Undefined: facebook_yoga_Unit = 0;
pub const facebook_yoga_Unit_Point: facebook_yoga_Unit = 1;
pub const facebook_yoga_Unit_Percent: facebook_yoga_Unit = 2;
pub const facebook_yoga_Unit_Auto: facebook_yoga_Unit = 3;
pub const facebook_yoga_Unit_MaxContent: facebook_yoga_Unit = 4;
pub const facebook_yoga_Unit_FitContent: facebook_yoga_Unit = 5;
pub const facebook_yoga_Unit_Stretch: facebook_yoga_Unit = 6;
pub fn ordinalCount_162() -> i32 {
    return 7;
}
pub fn scopedEnum_163(mut unscoped: YGUnit) -> facebook_yoga_Unit {
    return (unscoped as facebook_yoga_Unit);
}
pub fn unscopedEnum_164(mut scoped: facebook_yoga_Unit) -> YGUnit {
    return (scoped as YGUnit);
}
pub fn toString_165(mut e: facebook_yoga_Unit) -> Ptr<i8> {
    return ({ YGUnitToString_166(({ unscopedEnum_164(e) })) });
}
pub type facebook_yoga_Wrap = u8;
pub const facebook_yoga_Wrap_NoWrap: facebook_yoga_Wrap = 0;
pub const facebook_yoga_Wrap_Wrap: facebook_yoga_Wrap = 1;
pub const facebook_yoga_Wrap_WrapReverse: facebook_yoga_Wrap = 2;
pub fn ordinalCount_37() -> i32 {
    return 3;
}
pub fn scopedEnum_167(mut unscoped: YGWrap) -> facebook_yoga_Wrap {
    return (unscoped as facebook_yoga_Wrap);
}
pub fn unscopedEnum_168(mut scoped: facebook_yoga_Wrap) -> YGWrap {
    return (scoped as YGWrap);
}
pub fn toString_169(mut e: facebook_yoga_Wrap) -> Ptr<i8> {
    return ({ YGWrapToString_170(({ unscopedEnum_168(e) })) });
}
pub type facebook_yoga_GridLineType = u8;
pub const facebook_yoga_GridLineType_Auto: facebook_yoga_GridLineType = 0;
pub const facebook_yoga_GridLineType_Integer: facebook_yoga_GridLineType = 1;
pub const facebook_yoga_GridLineType_Span: facebook_yoga_GridLineType = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct facebook_yoga_GridLine {
    #[offset(0)]
    #[byte_size(1)]
    pub type_: facebook_yoga_GridLineType,
    #[offset(4)]
    pub integer: i32,
}
impl facebook_yoga_GridLine {
    pub fn auto_() -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Auto,
            integer: 0,
        };
    }
    pub fn fromInteger(mut value: i32) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Integer,
            integer: value,
        };
    }
    pub fn span(mut value: i32) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Span,
            integer: value,
        };
    }
}
impl CppEq for facebook_yoga_GridLine {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_GridLineImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for facebook_yoga_GridLine {
    fn default() -> Self {
        {
            facebook_yoga_GridLine {
                type_: <facebook_yoga_GridLineType>::default(),
                integer: 0_i32,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct facebook_yoga_StyleSizeLength {
    #[offset(0)]
    #[byte_size(4)]
    value_: facebook_yoga_FloatOptional,
    #[offset(4)]
    #[byte_size(1)]
    unit_: facebook_yoga_Unit,
}
impl facebook_yoga_StyleSizeLength {
    pub fn points(mut value: f32) -> facebook_yoga_StyleSizeLength {
        return if ({ isUndefined_105(value) }) || ({ isinf_109(value) }) {
            ({ facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Point },
            )
        };
    }
    pub fn percent(mut value: f32) -> facebook_yoga_StyleSizeLength {
        return if ({ isUndefined_105(value) }) || ({ isinf_109(value) }) {
            ({ facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Percent },
            )
        };
    }
    pub fn stretch(mut fraction: f32) -> facebook_yoga_StyleSizeLength {
        return if ({ isUndefined_105(fraction) }) || ({ isinf_109(fraction) }) {
            ({ facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ fraction }) },
                { facebook_yoga_Unit_Stretch },
            )
        };
    }
    pub fn ofAuto() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_Auto },
        );
    }
    pub fn ofMaxContent() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_MaxContent },
        );
    }
    pub fn ofFitContent() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_FitContent },
        );
    }
    pub fn ofStretch() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_Stretch },
        );
    }
    pub fn undefined() -> facebook_yoga_StyleSizeLength {
        return facebook_yoga_StyleSizeLength::new_1(
            { <facebook_yoga_FloatOptional>::default() },
            { facebook_yoga_Unit_Undefined },
        );
    }
    fn new_1(mut value: facebook_yoga_FloatOptional, mut unit: facebook_yoga_Unit) -> Self {
        Self {
            value_: (value).clone(),
            unit_: unit,
        }
    }
}
impl CppEq for facebook_yoga_StyleSizeLength {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_StyleSizeLengthImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for facebook_yoga_StyleSizeLength {
    fn default() -> Self {
        {
            facebook_yoga_StyleSizeLength {
                value_: <facebook_yoga_FloatOptional>::default(),
                unit_: facebook_yoga_Unit_Undefined,
            }
        }
    }
}
pub fn inexactEquals_171(
    a: Ptr<facebook_yoga_StyleSizeLength>,
    b: Ptr<facebook_yoga_StyleSizeLength>,
) -> bool {
    return ({
        let _other: Ptr<facebook_yoga_StyleSizeLength> = (b).clone();
        facebook_yoga_StyleSizeLengthImpl::inexactEquals(&a, _other)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(28)]
pub struct facebook_yoga_GridTrackSize {
    #[offset(0)]
    #[byte_size(8)]
    pub minSizingFunction: facebook_yoga_StyleSizeLength,
    #[offset(8)]
    #[byte_size(8)]
    pub maxSizingFunction: facebook_yoga_StyleSizeLength,
    #[offset(16)]
    pub baseSize: f32,
    #[offset(20)]
    pub growthLimit: f32,
    #[offset(24)]
    pub infinitelyGrowable: bool,
}
impl facebook_yoga_GridTrackSize {
    pub fn auto_() -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: ({ facebook_yoga_StyleSizeLength::ofAuto() }),
            maxSizingFunction: ({ facebook_yoga_StyleSizeLength::ofAuto() }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub fn length(mut points: f32) -> facebook_yoga_GridTrackSize {
        let mut len: facebook_yoga_StyleSizeLength =
            ({ facebook_yoga_StyleSizeLength::points(points) });
        return facebook_yoga_GridTrackSize {
            minSizingFunction: (len).clone(),
            maxSizingFunction: (len).clone(),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub fn fr(mut fraction: f32) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: ({ facebook_yoga_StyleSizeLength::ofAuto() }),
            maxSizingFunction: ({ facebook_yoga_StyleSizeLength::stretch(fraction) }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub fn percent(mut percentage: f32) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: ({ facebook_yoga_StyleSizeLength::percent(percentage) }),
            maxSizingFunction: ({ facebook_yoga_StyleSizeLength::percent(percentage) }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub fn minmax(
        mut min: facebook_yoga_StyleSizeLength,
        mut max: facebook_yoga_StyleSizeLength,
    ) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: (min).clone(),
            maxSizingFunction: (max).clone(),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
}
impl CppEq for facebook_yoga_GridTrackSize {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_GridTrackSizeImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for facebook_yoga_GridTrackSize {
    fn default() -> Self {
        {
            facebook_yoga_GridTrackSize {
                minSizingFunction: <facebook_yoga_StyleSizeLength>::default(),
                maxSizingFunction: <facebook_yoga_StyleSizeLength>::default(),
                baseSize: 0_f32,
                growthLimit: 0_f32,
                infinitelyGrowable: false,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(128)]
pub struct facebook_yoga_GridStyle {
    #[offset(0)]
    #[byte_size(24)]
    pub templateColumns: Vec<facebook_yoga_GridTrackSize>,
    #[offset(24)]
    #[byte_size(24)]
    pub templateRows: Vec<facebook_yoga_GridTrackSize>,
    #[offset(48)]
    #[byte_size(24)]
    pub autoColumns: Vec<facebook_yoga_GridTrackSize>,
    #[offset(72)]
    #[byte_size(24)]
    pub autoRows: Vec<facebook_yoga_GridTrackSize>,
    #[offset(96)]
    #[byte_size(8)]
    pub columnStart: facebook_yoga_GridLine,
    #[offset(104)]
    #[byte_size(8)]
    pub columnEnd: facebook_yoga_GridLine,
    #[offset(112)]
    #[byte_size(8)]
    pub rowStart: facebook_yoga_GridLine,
    #[offset(120)]
    #[byte_size(8)]
    pub rowEnd: facebook_yoga_GridLine,
}
impl facebook_yoga_GridStyle {
    pub fn move_from(_a0: Ptr<facebook_yoga_GridStyle>) -> Self {
        Self {
            templateColumns: field!(_a0, templateColumns).with_mut(std::mem::take),
            templateRows: field!(_a0, templateRows).with_mut(std::mem::take),
            autoColumns: field!(_a0, autoColumns).with_mut(std::mem::take),
            autoRows: field!(_a0, autoRows).with_mut(std::mem::take),
            columnStart: { (*_a0.upgrade().deref()).columnStart.clone() },
            columnEnd: { (*_a0.upgrade().deref()).columnEnd.clone() },
            rowStart: { (*_a0.upgrade().deref()).rowStart.clone() },
            rowEnd: { (*_a0.upgrade().deref()).rowEnd.clone() },
        }
    }
}
impl CppEq for facebook_yoga_GridStyle {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_GridStyleImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for facebook_yoga_GridStyle {
    fn default() -> Self {
        {
            facebook_yoga_GridStyle {
                templateColumns: Vec::new(),
                templateRows: Vec::new(),
                autoColumns: Vec::new(),
                autoRows: Vec::new(),
                columnStart: <facebook_yoga_GridLine>::default(),
                columnEnd: <facebook_yoga_GridLine>::default(),
                rowStart: <facebook_yoga_GridLine>::default(),
                rowEnd: <facebook_yoga_GridLine>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(8)]
pub struct facebook_yoga_GridStyleStorage {
    #[offset(0)]
    #[byte_size(8)]
    grid_: Option<Value<facebook_yoga_GridStyle>>,
}
impl facebook_yoga_GridStyleStorage {
    pub fn move_from(_a0: Ptr<facebook_yoga_GridStyleStorage>) -> Self {
        Self {
            grid_: field!(_a0, grid_)
                .with_mut(|__v: &mut Option<Value<facebook_yoga_GridStyle>>| __v.take()),
        }
    }
    pub fn copy_from(other: Ptr<facebook_yoga_GridStyleStorage>) -> Self {
        let __this: Value<facebook_yoga_GridStyleStorage> =
            Rc::new(RefCell::new(Self { grid_: None }));
        let this: Ptr<facebook_yoga_GridStyleStorage> = __this.as_pointer();
        ({
            let _other: Ptr<facebook_yoga_GridStyleStorage> = (other).clone();
            facebook_yoga_GridStyleStorageImpl::copy_assign(&this, _other)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn defaults() -> Ptr<facebook_yoga_GridStyle> {
        return kDefaults_172.with(|v| v.as_pointer());
    }
}
impl CppEq for facebook_yoga_GridStyleStorage {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_GridStyleStorageImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Clone for facebook_yoga_GridStyleStorage {
    fn clone(&self) -> Self {
        let __src: Value<facebook_yoga_GridStyleStorage> =
            Rc::new(RefCell::new(facebook_yoga_GridStyleStorage {
                grid_: self.grid_.clone(),
            }));
        facebook_yoga_GridStyleStorage::copy_from(__src.as_pointer())
    }
}
impl Default for facebook_yoga_GridStyleStorage {
    fn default() -> Self {
        { facebook_yoga_GridStyleStorage { grid_: None } }
    }
}
thread_local!(
    static kDefaults_172: Value<facebook_yoga_GridStyle> =
        Rc::new(RefCell::new(<facebook_yoga_GridStyle>::default()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct facebook_yoga_StyleLength {
    #[offset(0)]
    #[byte_size(4)]
    value_: facebook_yoga_FloatOptional,
    #[offset(4)]
    #[byte_size(1)]
    unit_: facebook_yoga_Unit,
}
impl facebook_yoga_StyleLength {
    pub fn points(mut value: f32) -> facebook_yoga_StyleLength {
        return if ({ isUndefined_105(value) }) || ({ isinf_109(value) }) {
            ({ facebook_yoga_StyleLength::undefined() })
        } else {
            facebook_yoga_StyleLength::new_1({ facebook_yoga_FloatOptional::new_1({ value }) }, {
                facebook_yoga_Unit_Point
            })
        };
    }
    pub fn percent(mut value: f32) -> facebook_yoga_StyleLength {
        return if ({ isUndefined_105(value) }) || ({ isinf_109(value) }) {
            ({ facebook_yoga_StyleLength::undefined() })
        } else {
            facebook_yoga_StyleLength::new_1({ facebook_yoga_FloatOptional::new_1({ value }) }, {
                facebook_yoga_Unit_Percent
            })
        };
    }
    pub fn ofAuto() -> facebook_yoga_StyleLength {
        return facebook_yoga_StyleLength::new_1({ <facebook_yoga_FloatOptional>::default() }, {
            facebook_yoga_Unit_Auto
        });
    }
    pub fn undefined() -> facebook_yoga_StyleLength {
        return facebook_yoga_StyleLength::new_1({ <facebook_yoga_FloatOptional>::default() }, {
            facebook_yoga_Unit_Undefined
        });
    }
    fn new_1(mut value: facebook_yoga_FloatOptional, mut unit: facebook_yoga_Unit) -> Self {
        Self {
            value_: (value).clone(),
            unit_: unit,
        }
    }
}
impl CppEq for facebook_yoga_StyleLength {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_StyleLengthImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for facebook_yoga_StyleLength {
    fn default() -> Self {
        {
            facebook_yoga_StyleLength {
                value_: <facebook_yoga_FloatOptional>::default(),
                unit_: facebook_yoga_Unit_Undefined,
            }
        }
    }
}
pub fn inexactEquals_173(
    a: Ptr<facebook_yoga_StyleLength>,
    b: Ptr<facebook_yoga_StyleLength>,
) -> bool {
    return ({
        let _other: Ptr<facebook_yoga_StyleLength> = (b).clone();
        facebook_yoga_StyleLengthImpl::inexactEquals(&a, _other)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(64)]
struct facebook_yoga_SmallValueBuffer_4__Overflow {
    #[offset(0)]
    #[byte_size(24)]
    pub buffer_: Vec<u32>,
    #[offset(24)]
    #[byte_size(40)]
    pub wideElements_: Vec<bool>,
}
impl facebook_yoga_SmallValueBuffer_4__Overflow {
    pub fn move_from(_a0: Ptr<facebook_yoga_SmallValueBuffer_4__Overflow>) -> Self {
        Self {
            buffer_: field!(_a0, buffer_).with_mut(std::mem::take),
            wideElements_: field!(_a0, wideElements_).with_mut(std::mem::take),
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(40)]
pub struct facebook_yoga_SmallValueBuffer_4_ {
    #[offset(0)]
    count_: u16,
    #[offset(4)]
    #[byte_size(16)]
    buffer_: Vec<u32>,
    #[offset(24)]
    #[byte_size(8)]
    wideElements_: Vec<bool>,
    #[offset(32)]
    #[byte_size(8)]
    overflow_: Option<Value<facebook_yoga_SmallValueBuffer_4__Overflow>>,
}
impl facebook_yoga_SmallValueBuffer_4_ {
    pub fn copy_from(other: Ptr<facebook_yoga_SmallValueBuffer_4_>) -> Self {
        let __this: Value<facebook_yoga_SmallValueBuffer_4_> = Rc::new(RefCell::new(Self {
            count_: 0_u16,
            buffer_: std::array::from_fn::<_, 4, _>(|_| 0_u32).to_vec(),
            wideElements_: vec![false; 4],
            overflow_: None,
        }));
        let this: Ptr<facebook_yoga_SmallValueBuffer_4_> = __this.as_pointer();
        ({
            let _other: Ptr<facebook_yoga_SmallValueBuffer_4_> = (other).clone();
            facebook_yoga_SmallValueBuffer_4_Impl::copy_assign(&this, _other)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<facebook_yoga_SmallValueBuffer_4_>) -> Self {
        Self {
            count_: { (*other.upgrade().deref()).count_ },
            buffer_: field!(other, buffer_).with_mut(std::mem::take),
            wideElements_: field!(other, wideElements_).with_mut(std::mem::take),
            overflow_: field!(other, overflow_).with_mut(
                |__v: &mut Option<Value<facebook_yoga_SmallValueBuffer_4__Overflow>>| __v.take(),
            ),
        }
    }
}
impl Clone for facebook_yoga_SmallValueBuffer_4_ {
    fn clone(&self) -> Self {
        let __src: Value<facebook_yoga_SmallValueBuffer_4_> =
            Rc::new(RefCell::new(facebook_yoga_SmallValueBuffer_4_ {
                count_: self.count_.clone(),
                buffer_: self.buffer_.clone(),
                wideElements_: self.wideElements_.clone(),
                overflow_: self.overflow_.clone(),
            }));
        facebook_yoga_SmallValueBuffer_4_::copy_from(__src.as_pointer())
    }
}
impl Default for facebook_yoga_SmallValueBuffer_4_ {
    fn default() -> Self {
        {
            facebook_yoga_SmallValueBuffer_4_ {
                count_: 0_u16,
                buffer_: std::array::from_fn::<_, 4, _>(|_| 0_u32).to_vec(),
                wideElements_: vec![false; 4],
                overflow_: None,
            }
        }
    }
}
thread_local!(
    static kHandleTypeMask_174: Value<u16> = Rc::new(RefCell::new(7_u16));
);
thread_local!(
    static kHandleIndexedMask_175: Value<u16> = Rc::new(RefCell::new(8_u16));
);
thread_local!(
    static kHandleValueMask_176: Value<u16> = Rc::new(RefCell::new(65520_u16));
);
pub type facebook_yoga_StyleValueHandle_Type = u8;
pub const facebook_yoga_StyleValueHandle_Type_Undefined: facebook_yoga_StyleValueHandle_Type = 0;
pub const facebook_yoga_StyleValueHandle_Type_Point: facebook_yoga_StyleValueHandle_Type = 1;
pub const facebook_yoga_StyleValueHandle_Type_Percent: facebook_yoga_StyleValueHandle_Type = 2;
pub const facebook_yoga_StyleValueHandle_Type_Number: facebook_yoga_StyleValueHandle_Type = 3;
pub const facebook_yoga_StyleValueHandle_Type_Auto: facebook_yoga_StyleValueHandle_Type = 4;
pub const facebook_yoga_StyleValueHandle_Type_Keyword: facebook_yoga_StyleValueHandle_Type = 5;
pub type facebook_yoga_StyleValueHandle_Keyword = u8;
pub const facebook_yoga_StyleValueHandle_Keyword_MaxContent:
    facebook_yoga_StyleValueHandle_Keyword = 0;
pub const facebook_yoga_StyleValueHandle_Keyword_FitContent:
    facebook_yoga_StyleValueHandle_Keyword = 1;
pub const facebook_yoga_StyleValueHandle_Keyword_Stretch: facebook_yoga_StyleValueHandle_Keyword =
    2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct facebook_yoga_StyleValueHandle {
    #[offset(0)]
    repr_: u16,
}
impl facebook_yoga_StyleValueHandle {
    pub fn ofAuto() -> facebook_yoga_StyleValueHandle {
        let handle: Value<facebook_yoga_StyleValueHandle> =
            Rc::new(RefCell::new(<facebook_yoga_StyleValueHandle>::default()));
        ({
            facebook_yoga_StyleValueHandleImpl::setType(
                &handle.as_pointer(),
                facebook_yoga_StyleValueHandle_Type_Auto,
            )
        });
        return (*handle.borrow()).clone();
    }
}
impl Default for facebook_yoga_StyleValueHandle {
    fn default() -> Self {
        { facebook_yoga_StyleValueHandle { repr_: 0_u16 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Default)]
#[byte_size(40)]
pub struct facebook_yoga_StyleValuePool {
    #[offset(0)]
    #[byte_size(40)]
    buffer_: facebook_yoga_SmallValueBuffer_4_,
}
impl facebook_yoga_StyleValuePool {
    fn isIntegerPackable(mut f: f32) -> bool {
        let mut kMaxInlineAbsValue: u16 = (((1 << 11) - 1) as u16);
        let mut i: i32 = (f as i32);
        return (((i as f32) == f) && (i >= -(kMaxInlineAbsValue as i32)))
            && (i <= (kMaxInlineAbsValue as i32));
    }
    fn packInlineInteger(mut value: f32) -> u16 {
        let mut isNegative: u16 = (if (value < 0_f32) { 1 } else { 0 } as u16);
        return ((((isNegative as i32) << 11)
            | ((value as i32)
                * (if ((isNegative as u32) != 0_u32) {
                    -1_i32
                } else {
                    1
                }))) as u16);
    }
    fn unpackInlineInteger(mut value: u16) -> f32 {
        let mut kValueSignMask: u16 = 2048_u16;
        let mut kValueMagnitudeMask: u16 = 2047_u16;
        let mut isNegative: bool = (((value as i32) & (kValueSignMask as i32)) != 0);
        return ((((value as i32) & (kValueMagnitudeMask as i32))
            * (if isNegative { -1_i32 } else { 1 })) as f32);
    }
    pub fn move_from(_a0: Ptr<facebook_yoga_StyleValuePool>) -> Self {
        Self {
            buffer_: facebook_yoga_SmallValueBuffer_4_::move_from({ field_ptr!(_a0, buffer_) }),
        }
    }
}
thread_local!(
    pub static DefaultFlexGrow_177: Value<f32> = Rc::new(RefCell::new(0_f32));
);
thread_local!(
    pub static DefaultFlexShrink_178: Value<f32> = Rc::new(RefCell::new(0_f32));
);
thread_local!(
    pub static WebDefaultFlexShrink_179: Value<f32> = Rc::new(RefCell::new(1_f32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(160)]
pub struct facebook_yoga_Style {
    #[offset(0)]
    #[byte_size(1)]
    direction_: facebook_yoga_Direction,
    #[offset(0)]
    #[byte_size(1)]
    flexDirection_: facebook_yoga_FlexDirection,
    #[offset(0)]
    #[byte_size(1)]
    justifyContent_: facebook_yoga_Justify,
    #[offset(1)]
    #[byte_size(1)]
    justifyItems_: facebook_yoga_Justify,
    #[offset(1)]
    #[byte_size(1)]
    justifySelf_: facebook_yoga_Justify,
    #[offset(2)]
    #[byte_size(1)]
    alignContent_: facebook_yoga_Align,
    #[offset(2)]
    #[byte_size(1)]
    alignItems_: facebook_yoga_Align,
    #[offset(3)]
    #[byte_size(1)]
    alignSelf_: facebook_yoga_Align,
    #[offset(3)]
    #[byte_size(1)]
    positionType_: facebook_yoga_PositionType,
    #[offset(3)]
    #[byte_size(1)]
    flexWrap_: facebook_yoga_Wrap,
    #[offset(4)]
    #[byte_size(1)]
    overflow_: facebook_yoga_Overflow,
    #[offset(4)]
    #[byte_size(1)]
    display_: facebook_yoga_Display,
    #[offset(4)]
    #[byte_size(1)]
    boxSizing_: facebook_yoga_BoxSizing,
    #[offset(5)]
    #[byte_size(2)]
    flex_: facebook_yoga_StyleValueHandle,
    #[offset(7)]
    #[byte_size(2)]
    flexGrow_: facebook_yoga_StyleValueHandle,
    #[offset(9)]
    #[byte_size(2)]
    flexShrink_: facebook_yoga_StyleValueHandle,
    #[offset(11)]
    #[byte_size(2)]
    flexBasis_: facebook_yoga_StyleValueHandle,
    #[offset(13)]
    #[byte_size(18)]
    margin_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(31)]
    #[byte_size(18)]
    position_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(49)]
    #[byte_size(18)]
    padding_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(67)]
    #[byte_size(18)]
    border_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(85)]
    #[byte_size(6)]
    gap_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(91)]
    #[byte_size(4)]
    dimensions_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(95)]
    #[byte_size(4)]
    minDimensions_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(99)]
    #[byte_size(4)]
    maxDimensions_: Vec<facebook_yoga_StyleValueHandle>,
    #[offset(103)]
    #[byte_size(2)]
    aspectRatio_: facebook_yoga_StyleValueHandle,
    #[offset(112)]
    #[byte_size(8)]
    grid_: facebook_yoga_GridStyleStorage,
    #[offset(120)]
    #[byte_size(40)]
    pool_: facebook_yoga_StyleValuePool,
}
impl facebook_yoga_Style {
    fn numbersEqual(
        lhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::isUndefined(&lhsHandle) })
            && ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&rhsHandle) }))
            || ({
                let _lhs: facebook_yoga_FloatOptional = ({
                    let _handle: facebook_yoga_StyleValueHandle =
                        (*lhsHandle.upgrade().deref()).clone();
                    facebook_yoga_StyleValuePoolImpl::getNumber(&lhsPool, _handle)
                });
                let _rhs: facebook_yoga_FloatOptional = ({
                    let _handle: facebook_yoga_StyleValueHandle =
                        (*rhsHandle.upgrade().deref()).clone();
                    facebook_yoga_StyleValuePoolImpl::getNumber(&rhsPool, _handle)
                });
                operator_eq_114(_lhs, _rhs)
            });
    }
    fn lengthsEqual(
        lhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::isUndefined(&lhsHandle) })
            && ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&rhsHandle) }))
            || ({
                let _rhs: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle =
                            (*rhsHandle.upgrade().deref()).clone();
                        facebook_yoga_StyleValuePoolImpl::getLength(&rhsPool, _handle)
                    }),
                ));
                facebook_yoga_StyleLengthImpl::operator_eq(
                    &Rc::new(RefCell::new(
                        ({
                            let _handle: facebook_yoga_StyleValueHandle =
                                (*lhsHandle.upgrade().deref()).clone();
                            facebook_yoga_StyleValuePoolImpl::getLength(&lhsPool, _handle)
                        }),
                    ))
                    .as_pointer(),
                    _rhs.as_pointer(),
                )
            });
    }
    fn sizeLengthsEqual(
        lhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::isUndefined(&lhsHandle) })
            && ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&rhsHandle) }))
            || ({
                let _rhs: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle =
                            (*rhsHandle.upgrade().deref()).clone();
                        facebook_yoga_StyleValuePoolImpl::getSize(&rhsPool, _handle)
                    }),
                ));
                facebook_yoga_StyleSizeLengthImpl::operator_eq(
                    &Rc::new(RefCell::new(
                        ({
                            let _handle: facebook_yoga_StyleValueHandle =
                                (*lhsHandle.upgrade().deref()).clone();
                            facebook_yoga_StyleValuePoolImpl::getSize(&lhsPool, _handle)
                        }),
                    ))
                    .as_pointer(),
                    _rhs.as_pointer(),
                )
            });
    }
    pub fn move_from(_a0: Ptr<facebook_yoga_Style>) -> Self {
        Self {
            direction_: { (*_a0.upgrade().deref()).direction_ },
            flexDirection_: { (*_a0.upgrade().deref()).flexDirection_ },
            justifyContent_: { (*_a0.upgrade().deref()).justifyContent_ },
            justifyItems_: { (*_a0.upgrade().deref()).justifyItems_ },
            justifySelf_: { (*_a0.upgrade().deref()).justifySelf_ },
            alignContent_: { (*_a0.upgrade().deref()).alignContent_ },
            alignItems_: { (*_a0.upgrade().deref()).alignItems_ },
            alignSelf_: { (*_a0.upgrade().deref()).alignSelf_ },
            positionType_: { (*_a0.upgrade().deref()).positionType_ },
            flexWrap_: { (*_a0.upgrade().deref()).flexWrap_ },
            overflow_: { (*_a0.upgrade().deref()).overflow_ },
            display_: { (*_a0.upgrade().deref()).display_ },
            boxSizing_: { (*_a0.upgrade().deref()).boxSizing_ },
            flex_: { (*_a0.upgrade().deref()).flex_.clone() },
            flexGrow_: { (*_a0.upgrade().deref()).flexGrow_.clone() },
            flexShrink_: { (*_a0.upgrade().deref()).flexShrink_.clone() },
            flexBasis_: { (*_a0.upgrade().deref()).flexBasis_.clone() },
            margin_: field!(_a0, margin_).with_mut(std::mem::take),
            position_: field!(_a0, position_).with_mut(std::mem::take),
            padding_: field!(_a0, padding_).with_mut(std::mem::take),
            border_: field!(_a0, border_).with_mut(std::mem::take),
            gap_: field!(_a0, gap_).with_mut(std::mem::take),
            dimensions_: field!(_a0, dimensions_).with_mut(std::mem::take),
            minDimensions_: field!(_a0, minDimensions_).with_mut(std::mem::take),
            maxDimensions_: field!(_a0, maxDimensions_).with_mut(std::mem::take),
            aspectRatio_: { (*_a0.upgrade().deref()).aspectRatio_.clone() },
            grid_: facebook_yoga_GridStyleStorage::move_from({ field_ptr!(_a0, grid_) }),
            pool_: facebook_yoga_StyleValuePool::move_from({ field_ptr!(_a0, pool_) }),
        }
    }
    fn anyIsPercent_121_9(handles: Ptr<Vec<facebook_yoga_StyleValueHandle>>) -> bool {
        return {
            let mut __it = handles.decay();
            let __last = handles.decay().to_end();
            let __pred = FnPtr::<fn(Ptr<facebook_yoga_StyleValueHandle>) -> bool>::new(
                |handle: Ptr<facebook_yoga_StyleValueHandle>| -> bool {
                    {
                        return ({ facebook_yoga_StyleValueHandleImpl::isPercent(&handle) });
                    }
                },
            );
            let mut __found = false;
            while !__found && __it != __last {
                if __pred.call(__it.iter_deref()) {
                    __found = true;
                } else {
                    __it.iter_inc();
                }
            }
            __found
        };
    }
    fn lengthsEqual_124_9(
        lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> bool {
        return {
            let mut __a = lhs.decay();
            let __a_end = lhs.decay().to_end();
            let mut __b = rhs.decay();
            let __b_end = rhs.decay().to_end();
            let __pred = lambda!(
                Generic,
                {
                    let lhsPool: Ptr<facebook_yoga_StyleValuePool> = (lhsPool).clone();
                    let rhsPool: Ptr<facebook_yoga_StyleValuePool> = (rhsPool).clone();
                },
                |lhs: Ptr<facebook_yoga_StyleValueHandle>,
                 rhs: Ptr<facebook_yoga_StyleValueHandle>|
                 -> bool {
                    return ({
                        let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> = (lhs).clone();
                        let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = (lhsPool).clone();
                        let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> = (rhs).clone();
                        let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = (rhsPool).clone();
                        facebook_yoga_Style::lengthsEqual(
                            _lhsHandle, _lhsPool, _rhsHandle, _rhsPool,
                        )
                    });
                }
            );
            let mut __eq = true;
            while __eq && __a != __a_end && __b != __b_end {
                if __pred
                    .spec::<fn(
                        Ptr<<Ptr<facebook_yoga_StyleValueHandle> as IterElem>::Elem>,
                        Ptr<<Ptr<facebook_yoga_StyleValueHandle> as IterElem>::Elem>,
                    ) -> bool>(0)
                    .call(__a.iter_deref(), __b.iter_deref())
                {
                    __a.iter_inc();
                    __b.iter_inc();
                } else {
                    __eq = false;
                }
            }
            __eq && __a == __a_end && __b == __b_end
        };
    }
    fn lengthsEqual_124_3(
        lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> bool {
        return {
            let mut __a = lhs.decay();
            let __a_end = lhs.decay().to_end();
            let mut __b = rhs.decay();
            let __b_end = rhs.decay().to_end();
            let __pred = lambda!(
                Generic,
                {
                    let lhsPool: Ptr<facebook_yoga_StyleValuePool> = (lhsPool).clone();
                    let rhsPool: Ptr<facebook_yoga_StyleValuePool> = (rhsPool).clone();
                },
                |lhs: Ptr<facebook_yoga_StyleValueHandle>,
                 rhs: Ptr<facebook_yoga_StyleValueHandle>|
                 -> bool {
                    return ({
                        let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> = (lhs).clone();
                        let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = (lhsPool).clone();
                        let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> = (rhs).clone();
                        let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = (rhsPool).clone();
                        facebook_yoga_Style::lengthsEqual(
                            _lhsHandle, _lhsPool, _rhsHandle, _rhsPool,
                        )
                    });
                }
            );
            let mut __eq = true;
            while __eq && __a != __a_end && __b != __b_end {
                if __pred
                    .spec::<fn(
                        Ptr<<Ptr<facebook_yoga_StyleValueHandle> as IterElem>::Elem>,
                        Ptr<<Ptr<facebook_yoga_StyleValueHandle> as IterElem>::Elem>,
                    ) -> bool>(0)
                    .call(__a.iter_deref(), __b.iter_deref())
                {
                    __a.iter_inc();
                    __b.iter_inc();
                } else {
                    __eq = false;
                }
            }
            __eq && __a == __a_end && __b == __b_end
        };
    }
    fn sizeLengthsEqual_126_2(
        lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> bool {
        return {
            let mut __a = lhs.decay();
            let __a_end = lhs.decay().to_end();
            let mut __b = rhs.decay();
            let __b_end = rhs.decay().to_end();
            let __pred = lambda!(
                Generic,
                {
                    let lhsPool: Ptr<facebook_yoga_StyleValuePool> = (lhsPool).clone();
                    let rhsPool: Ptr<facebook_yoga_StyleValuePool> = (rhsPool).clone();
                },
                |lhs: Ptr<facebook_yoga_StyleValueHandle>,
                 rhs: Ptr<facebook_yoga_StyleValueHandle>|
                 -> bool {
                    return ({
                        let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> = (lhs).clone();
                        let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = (lhsPool).clone();
                        let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> = (rhs).clone();
                        let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = (rhsPool).clone();
                        facebook_yoga_Style::sizeLengthsEqual(
                            _lhsHandle, _lhsPool, _rhsHandle, _rhsPool,
                        )
                    });
                }
            );
            let mut __eq = true;
            while __eq && __a != __a_end && __b != __b_end {
                if __pred
                    .spec::<fn(
                        Ptr<<Ptr<facebook_yoga_StyleValueHandle> as IterElem>::Elem>,
                        Ptr<<Ptr<facebook_yoga_StyleValueHandle> as IterElem>::Elem>,
                    ) -> bool>(0)
                    .call(__a.iter_deref(), __b.iter_deref())
                {
                    __a.iter_inc();
                    __b.iter_inc();
                } else {
                    __eq = false;
                }
            }
            __eq && __a == __a_end && __b == __b_end
        };
    }
}
impl CppEq for facebook_yoga_Style {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        facebook_yoga_StyleImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for facebook_yoga_Style {
    fn default() -> Self {
        {
            facebook_yoga_Style {
                direction_: facebook_yoga_Direction_Inherit,
                flexDirection_: facebook_yoga_FlexDirection_Column,
                justifyContent_: facebook_yoga_Justify_FlexStart,
                justifyItems_: facebook_yoga_Justify_Stretch,
                justifySelf_: facebook_yoga_Justify_Auto,
                alignContent_: facebook_yoga_Align_FlexStart,
                alignItems_: facebook_yoga_Align_Stretch,
                alignSelf_: facebook_yoga_Align_Auto,
                positionType_: facebook_yoga_PositionType_Relative,
                flexWrap_: facebook_yoga_Wrap_NoWrap,
                overflow_: facebook_yoga_Overflow_Visible,
                display_: facebook_yoga_Display_Flex,
                boxSizing_: facebook_yoga_BoxSizing_BorderBox,
                flex_: <facebook_yoga_StyleValueHandle>::default(),
                flexGrow_: <facebook_yoga_StyleValueHandle>::default(),
                flexShrink_: <facebook_yoga_StyleValueHandle>::default(),
                flexBasis_: ({ facebook_yoga_StyleValueHandle::ofAuto() }),
                margin_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                position_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                padding_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                border_: std::array::from_fn::<_, 9, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                gap_: std::array::from_fn::<_, 3, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                dimensions_: Vec::from([
                    ({ facebook_yoga_StyleValueHandle::ofAuto() }),
                    ({ facebook_yoga_StyleValueHandle::ofAuto() }),
                ]),
                minDimensions_: std::array::from_fn::<_, 2, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                maxDimensions_: std::array::from_fn::<_, 2, _>(|_| {
                    <facebook_yoga_StyleValueHandle>::default()
                })
                .to_vec(),
                aspectRatio_: <facebook_yoga_StyleValueHandle>::default(),
                grid_: <facebook_yoga_GridStyleStorage>::default(),
                pool_: <facebook_yoga_StyleValuePool>::default(),
            }
        }
    }
}
pub type YGNode = facebook_yoga_Node;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(720)]
pub struct facebook_yoga_Node {
    #[offset(0)]
    hasNewLayout_: bool,
    #[offset(0)]
    isReferenceBaseline_: bool,
    #[offset(0)]
    isDirty_: bool,
    #[offset(0)]
    alwaysFormsContainingBlock_: bool,
    #[offset(0)]
    #[byte_size(1)]
    nodeType_: facebook_yoga_NodeType,
    #[offset(8)]
    #[byte_size(8)]
    context_: AnyPtr,
    #[offset(16)]
    #[byte_size(8)]
    measureFunc_: FnPtr<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
    #[offset(24)]
    #[byte_size(8)]
    minContentMeasureFunc_:
        FnPtr<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
    #[offset(32)]
    #[byte_size(4)]
    minContentWidth_: facebook_yoga_FloatOptional,
    #[offset(36)]
    #[byte_size(4)]
    minContentHeight_: facebook_yoga_FloatOptional,
    #[offset(40)]
    #[byte_size(8)]
    baselineFunc_: FnPtr<fn(Ptr<YGNode>, f32, f32) -> f32>,
    #[offset(48)]
    #[byte_size(8)]
    dirtiedFunc_: FnPtr<fn(Ptr<YGNode>)>,
    #[offset(56)]
    #[byte_size(160)]
    style_: facebook_yoga_Style,
    #[offset(216)]
    #[byte_size(432)]
    layout_: facebook_yoga_LayoutResults,
    #[offset(648)]
    lineIndex_: usize,
    #[offset(656)]
    contentsChildrenCount_: usize,
    #[offset(664)]
    #[byte_size(8)]
    owner_: Ptr<facebook_yoga_Node>,
    #[offset(672)]
    #[byte_size(24)]
    children_: Vec<Ptr<facebook_yoga_Node>>,
    #[offset(696)]
    #[byte_size(8)]
    config_: Ptr<facebook_yoga_Config>,
    #[offset(704)]
    #[byte_size(16)]
    processedDimensions_: Vec<facebook_yoga_StyleSizeLength>,
}
impl facebook_yoga_Node {}
impl Default for facebook_yoga_Node {
    fn default() -> Self {
        {
            facebook_yoga_Node {
                hasNewLayout_: true,
                isReferenceBaseline_: false,
                isDirty_: true,
                alwaysFormsContainingBlock_: false,
                nodeType_: facebook_yoga_NodeType_Default,
                context_: AnyPtr::default(),
                measureFunc_: FnPtr::<
                    fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
                >::null(),
                minContentMeasureFunc_: FnPtr::<
                    fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
                >::null(),
                minContentWidth_: <facebook_yoga_FloatOptional>::default(),
                minContentHeight_: <facebook_yoga_FloatOptional>::default(),
                baselineFunc_: FnPtr::<fn(Ptr<YGNode>, f32, f32) -> f32>::null(),
                dirtiedFunc_: FnPtr::<fn(Ptr<YGNode>)>::null(),
                style_: <facebook_yoga_Style>::default(),
                layout_: <facebook_yoga_LayoutResults>::default(),
                lineIndex_: 0_usize,
                contentsChildrenCount_: 0_usize,
                owner_: Ptr::<facebook_yoga_Node>::null(),
                children_: Default::default(),
                config_: Ptr::<facebook_yoga_Config>::null(),
                processedDimensions_: Vec::from([
                    ({ facebook_yoga_StyleSizeLength::undefined() }),
                    ({ facebook_yoga_StyleSizeLength::undefined() }),
                ]),
            }
        }
    }
}
pub fn resolveRef_180(mut ref_: Ptr<YGNode>) -> Ptr<facebook_yoga_Node> {
    return ref_.reinterpret_cast::<facebook_yoga_Node>();
}
pub fn resolveRef_181(mut ref_: Ptr<YGNode>) -> Ptr<facebook_yoga_Node> {
    return ref_.reinterpret_cast::<facebook_yoga_Node>();
}
pub fn YGConfigNew_182() -> Ptr<YGConfig> {
    return Ptr::alloc(facebook_yoga_Config::new({ ({ getDefaultLogger_183() }) }))
        .reinterpret_cast::<YGConfig>();
}
pub fn YGConfigFree_184(mut config: Ptr<YGConfig>) {
    ({ resolveRef_72((config).clone()) }).delete();
}
pub fn YGConfigGetDefault_185() -> Ptr<YGConfig> {
    return ({ facebook_yoga_Config::getDefault() }).reinterpret_cast::<YGConfig>();
}
pub fn YGConfigSetUseWebDefaults_186(mut config: Ptr<YGConfig>, mut enabled: bool) {
    ({
        facebook_yoga_ConfigImpl::setUseWebDefaults(&({ resolveRef_72((config).clone()) }), enabled)
    });
}
pub fn YGConfigGetUseWebDefaults_187(mut config: Ptr<YGConfig>) -> bool {
    return ({ facebook_yoga_ConfigImpl::useWebDefaults(&({ resolveRef_73(config) })) });
}
pub fn YGConfigSetPointScaleFactor_188(mut config: Ptr<YGConfig>, mut pixelsInPoint: f32) {
    ({
        assertFatalWithConfig_189(
            ({ resolveRef_72((config).clone()) }),
            (pixelsInPoint >= 0_f32),
            Ptr::<i8>::from_string_literal(b"Scale factor should not be less than zero"),
        )
    });
    ({
        facebook_yoga_ConfigImpl::setPointScaleFactor(
            &({ resolveRef_72((config).clone()) }),
            pixelsInPoint,
        )
    });
}
pub fn YGConfigGetPointScaleFactor_190(mut config: Ptr<YGConfig>) -> f32 {
    return ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&({ resolveRef_73(config) })) });
}
pub fn YGConfigSetErrata_191(mut config: Ptr<YGConfig>, mut errata: YGErrata) {
    ({
        facebook_yoga_ConfigImpl::setErrata(
            &({ resolveRef_72((config).clone()) }),
            ({ scopedEnum_57(errata) }),
        )
    });
}
pub fn YGConfigGetErrata_192(mut config: Ptr<YGConfig>) -> YGErrata {
    return ({
        unscopedEnum_58(({ facebook_yoga_ConfigImpl::getErrata(&({ resolveRef_73(config) })) }))
    });
}
pub fn YGConfigSetLogger_193(
    mut config: Ptr<YGConfig>,
    mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>,
) {
    if !((logger).is_null()) {
        ({
            facebook_yoga_ConfigImpl::setLogger(
                &({ resolveRef_72((config).clone()) }),
                (logger).clone(),
            )
        });
    } else {
        ({
            facebook_yoga_ConfigImpl::setLogger(
                &({ resolveRef_72((config).clone()) }),
                ({ getDefaultLogger_183() }),
            )
        });
    }
}
pub fn YGConfigSetContext_194(mut config: Ptr<YGConfig>, mut context: AnyPtr) {
    ({
        facebook_yoga_ConfigImpl::setContext(
            &({ resolveRef_72((config).clone()) }),
            (context).clone(),
        )
    });
}
pub fn YGConfigGetContext_195(mut config: Ptr<YGConfig>) -> AnyPtr {
    return ({ facebook_yoga_ConfigImpl::getContext(&({ resolveRef_73(config) })) });
}
pub fn YGConfigSetExperimentalFeatureEnabled_196(
    mut config: Ptr<YGConfig>,
    mut feature: YGExperimentalFeature,
    mut enabled: bool,
) {
    ({
        facebook_yoga_ConfigImpl::setExperimentalFeatureEnabled(
            &({ resolveRef_72((config).clone()) }),
            ({ scopedEnum_62(feature) }),
            enabled,
        )
    });
}
pub fn YGConfigIsExperimentalFeatureEnabled_197(
    mut config: Ptr<YGConfig>,
    mut feature: YGExperimentalFeature,
) -> bool {
    return ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ resolveRef_73(config) }),
            ({ scopedEnum_62(feature) }),
        )
    });
}
pub fn YGConfigSetCloneNodeFunc_198(
    mut config: Ptr<YGConfig>,
    mut callback: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, usize) -> Ptr<YGNode>>,
) {
    ({
        facebook_yoga_ConfigImpl::setCloneNodeCallback(
            &({ resolveRef_72((config).clone()) }),
            (callback).clone(),
        )
    });
}
pub fn YGAlignToString_140(mut value: YGAlign) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGAlign_YGAlignAuto as i32) => {
                return Ptr::<i8>::from_string_literal(b"auto");
            }
            __v if __v == (YGAlign_YGAlignFlexStart as i32) => {
                return Ptr::<i8>::from_string_literal(b"flex-start");
            }
            __v if __v == (YGAlign_YGAlignCenter as i32) => {
                return Ptr::<i8>::from_string_literal(b"center");
            }
            __v if __v == (YGAlign_YGAlignFlexEnd as i32) => {
                return Ptr::<i8>::from_string_literal(b"flex-end");
            }
            __v if __v == (YGAlign_YGAlignStretch as i32) => {
                return Ptr::<i8>::from_string_literal(b"stretch");
            }
            __v if __v == (YGAlign_YGAlignBaseline as i32) => {
                return Ptr::<i8>::from_string_literal(b"baseline");
            }
            __v if __v == (YGAlign_YGAlignSpaceBetween as i32) => {
                return Ptr::<i8>::from_string_literal(b"space-between");
            }
            __v if __v == (YGAlign_YGAlignSpaceAround as i32) => {
                return Ptr::<i8>::from_string_literal(b"space-around");
            }
            __v if __v == (YGAlign_YGAlignSpaceEvenly as i32) => {
                return Ptr::<i8>::from_string_literal(b"space-evenly");
            }
            __v if __v == (YGAlign_YGAlignStart as i32) => {
                return Ptr::<i8>::from_string_literal(b"start");
            }
            __v if __v == (YGAlign_YGAlignEnd as i32) => {
                return Ptr::<i8>::from_string_literal(b"end");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGBoxSizingToString_144(mut value: YGBoxSizing) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGBoxSizing_YGBoxSizingBorderBox as i32) => {
                return Ptr::<i8>::from_string_literal(b"border-box");
            }
            __v if __v == (YGBoxSizing_YGBoxSizingContentBox as i32) => {
                return Ptr::<i8>::from_string_literal(b"content-box");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGDimensionToString_83(mut value: YGDimension) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGDimension_YGDimensionWidth as i32) => {
                return Ptr::<i8>::from_string_literal(b"width");
            }
            __v if __v == (YGDimension_YGDimensionHeight as i32) => {
                return Ptr::<i8>::from_string_literal(b"height");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGDirectionToString_87(mut value: YGDirection) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGDirection_YGDirectionInherit as i32) => {
                return Ptr::<i8>::from_string_literal(b"inherit");
            }
            __v if __v == (YGDirection_YGDirectionLTR as i32) => {
                return Ptr::<i8>::from_string_literal(b"ltr");
            }
            __v if __v == (YGDirection_YGDirectionRTL as i32) => {
                return Ptr::<i8>::from_string_literal(b"rtl");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGDisplayToString_77(mut value: YGDisplay) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGDisplay_YGDisplayFlex as i32) => {
                return Ptr::<i8>::from_string_literal(b"flex");
            }
            __v if __v == (YGDisplay_YGDisplayNone as i32) => {
                return Ptr::<i8>::from_string_literal(b"none");
            }
            __v if __v == (YGDisplay_YGDisplayContents as i32) => {
                return Ptr::<i8>::from_string_literal(b"contents");
            }
            __v if __v == (YGDisplay_YGDisplayGrid as i32) => {
                return Ptr::<i8>::from_string_literal(b"grid");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGEdgeToString_92(mut value: YGEdge) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGEdge_YGEdgeLeft as i32) => {
                return Ptr::<i8>::from_string_literal(b"left");
            }
            __v if __v == (YGEdge_YGEdgeTop as i32) => {
                return Ptr::<i8>::from_string_literal(b"top");
            }
            __v if __v == (YGEdge_YGEdgeRight as i32) => {
                return Ptr::<i8>::from_string_literal(b"right");
            }
            __v if __v == (YGEdge_YGEdgeBottom as i32) => {
                return Ptr::<i8>::from_string_literal(b"bottom");
            }
            __v if __v == (YGEdge_YGEdgeStart as i32) => {
                return Ptr::<i8>::from_string_literal(b"start");
            }
            __v if __v == (YGEdge_YGEdgeEnd as i32) => {
                return Ptr::<i8>::from_string_literal(b"end");
            }
            __v if __v == (YGEdge_YGEdgeHorizontal as i32) => {
                return Ptr::<i8>::from_string_literal(b"horizontal");
            }
            __v if __v == (YGEdge_YGEdgeVertical as i32) => {
                return Ptr::<i8>::from_string_literal(b"vertical");
            }
            __v if __v == (YGEdge_YGEdgeAll as i32) => {
                return Ptr::<i8>::from_string_literal(b"all");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGErrataToString_60(mut value: YGErrata) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGErrata_YGErrataNone as i32) => {
                return Ptr::<i8>::from_string_literal(b"none");
            }
            __v if __v == (YGErrata_YGErrataStretchFlexBasis as i32) => {
                return Ptr::<i8>::from_string_literal(b"stretch-flex-basis");
            }
            __v if __v
                == (YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding as i32) =>
            {
                return Ptr::<i8>::from_string_literal(
                    b"absolute-position-without-insets-excludes-padding",
                );
            }
            __v if __v == (YGErrata_YGErrataAbsolutePercentAgainstInnerSize as i32) => {
                return Ptr::<i8>::from_string_literal(b"absolute-percent-against-inner-size");
            }
            __v if __v == (YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto as i32) => {
                return Ptr::<i8>::from_string_literal(b"min-size-undefined-instead-of-auto");
            }
            __v if __v == (YGErrata_YGErrataFlexFirstPassUsesRunningTotals as i32) => {
                return Ptr::<i8>::from_string_literal(b"flex-first-pass-uses-running-totals");
            }
            __v if __v == (YGErrata_YGErrataAll as i32) => {
                return Ptr::<i8>::from_string_literal(b"all");
            }
            __v if __v == (YGErrata_YGErrataClassic as i32) => {
                return Ptr::<i8>::from_string_literal(b"classic");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGExperimentalFeatureToString_65(mut value: YGExperimentalFeature) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis as i32) => {
                return Ptr::<i8>::from_string_literal(b"web-flex-basis");
            }
            __v if __v
                == (YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent as i32) =>
            {
                return Ptr::<i8>::from_string_literal(b"fix-flex-basis-fit-content");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGFlexDirectionToString_127(mut value: YGFlexDirection) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGFlexDirection_YGFlexDirectionColumn as i32) => {
                return Ptr::<i8>::from_string_literal(b"column");
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionColumnReverse as i32) => {
                return Ptr::<i8>::from_string_literal(b"column-reverse");
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRow as i32) => {
                return Ptr::<i8>::from_string_literal(b"row");
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRowReverse as i32) => {
                return Ptr::<i8>::from_string_literal(b"row-reverse");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGGridTrackTypeToString_199(mut value: YGGridTrackType) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as i32) => {
                return Ptr::<i8>::from_string_literal(b"auto");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as i32) => {
                return Ptr::<i8>::from_string_literal(b"points");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as i32) => {
                return Ptr::<i8>::from_string_literal(b"percent");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as i32) => {
                return Ptr::<i8>::from_string_literal(b"fr");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as i32) => {
                return Ptr::<i8>::from_string_literal(b"minmax");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGGutterToString_149(mut value: YGGutter) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGGutter_YGGutterColumn as i32) => {
                return Ptr::<i8>::from_string_literal(b"column");
            }
            __v if __v == (YGGutter_YGGutterRow as i32) => {
                return Ptr::<i8>::from_string_literal(b"row");
            }
            __v if __v == (YGGutter_YGGutterAll as i32) => {
                return Ptr::<i8>::from_string_literal(b"all");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGJustifyToString_153(mut value: YGJustify) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGJustify_YGJustifyAuto as i32) => {
                return Ptr::<i8>::from_string_literal(b"auto");
            }
            __v if __v == (YGJustify_YGJustifyFlexStart as i32) => {
                return Ptr::<i8>::from_string_literal(b"flex-start");
            }
            __v if __v == (YGJustify_YGJustifyCenter as i32) => {
                return Ptr::<i8>::from_string_literal(b"center");
            }
            __v if __v == (YGJustify_YGJustifyFlexEnd as i32) => {
                return Ptr::<i8>::from_string_literal(b"flex-end");
            }
            __v if __v == (YGJustify_YGJustifySpaceBetween as i32) => {
                return Ptr::<i8>::from_string_literal(b"space-between");
            }
            __v if __v == (YGJustify_YGJustifySpaceAround as i32) => {
                return Ptr::<i8>::from_string_literal(b"space-around");
            }
            __v if __v == (YGJustify_YGJustifySpaceEvenly as i32) => {
                return Ptr::<i8>::from_string_literal(b"space-evenly");
            }
            __v if __v == (YGJustify_YGJustifyStretch as i32) => {
                return Ptr::<i8>::from_string_literal(b"stretch");
            }
            __v if __v == (YGJustify_YGJustifyStart as i32) => {
                return Ptr::<i8>::from_string_literal(b"start");
            }
            __v if __v == (YGJustify_YGJustifyEnd as i32) => {
                return Ptr::<i8>::from_string_literal(b"end");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGLogLevelToString_70(mut value: YGLogLevel) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGLogLevel_YGLogLevelError as i32) => {
                return Ptr::<i8>::from_string_literal(b"error");
            }
            __v if __v == (YGLogLevel_YGLogLevelWarn as i32) => {
                return Ptr::<i8>::from_string_literal(b"warn");
            }
            __v if __v == (YGLogLevel_YGLogLevelInfo as i32) => {
                return Ptr::<i8>::from_string_literal(b"info");
            }
            __v if __v == (YGLogLevel_YGLogLevelDebug as i32) => {
                return Ptr::<i8>::from_string_literal(b"debug");
            }
            __v if __v == (YGLogLevel_YGLogLevelVerbose as i32) => {
                return Ptr::<i8>::from_string_literal(b"verbose");
            }
            __v if __v == (YGLogLevel_YGLogLevelFatal as i32) => {
                return Ptr::<i8>::from_string_literal(b"fatal");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGMeasureModeToString_97(mut value: YGMeasureMode) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGMeasureMode_YGMeasureModeUndefined as i32) => {
                return Ptr::<i8>::from_string_literal(b"undefined");
            }
            __v if __v == (YGMeasureMode_YGMeasureModeExactly as i32) => {
                return Ptr::<i8>::from_string_literal(b"exactly");
            }
            __v if __v == (YGMeasureMode_YGMeasureModeAtMost as i32) => {
                return Ptr::<i8>::from_string_literal(b"at-most");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGNodeTypeToString_101(mut value: YGNodeType) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGNodeType_YGNodeTypeDefault as i32) => {
                return Ptr::<i8>::from_string_literal(b"default");
            }
            __v if __v == (YGNodeType_YGNodeTypeText as i32) => {
                return Ptr::<i8>::from_string_literal(b"text");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGOverflowToString_157(mut value: YGOverflow) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGOverflow_YGOverflowVisible as i32) => {
                return Ptr::<i8>::from_string_literal(b"visible");
            }
            __v if __v == (YGOverflow_YGOverflowHidden as i32) => {
                return Ptr::<i8>::from_string_literal(b"hidden");
            }
            __v if __v == (YGOverflow_YGOverflowScroll as i32) => {
                return Ptr::<i8>::from_string_literal(b"scroll");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGPositionTypeToString_161(mut value: YGPositionType) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGPositionType_YGPositionTypeStatic as i32) => {
                return Ptr::<i8>::from_string_literal(b"static");
            }
            __v if __v == (YGPositionType_YGPositionTypeRelative as i32) => {
                return Ptr::<i8>::from_string_literal(b"relative");
            }
            __v if __v == (YGPositionType_YGPositionTypeAbsolute as i32) => {
                return Ptr::<i8>::from_string_literal(b"absolute");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGUnitToString_166(mut value: YGUnit) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGUnit_YGUnitUndefined as i32) => {
                return Ptr::<i8>::from_string_literal(b"undefined");
            }
            __v if __v == (YGUnit_YGUnitPoint as i32) => {
                return Ptr::<i8>::from_string_literal(b"point");
            }
            __v if __v == (YGUnit_YGUnitPercent as i32) => {
                return Ptr::<i8>::from_string_literal(b"percent");
            }
            __v if __v == (YGUnit_YGUnitAuto as i32) => {
                return Ptr::<i8>::from_string_literal(b"auto");
            }
            __v if __v == (YGUnit_YGUnitMaxContent as i32) => {
                return Ptr::<i8>::from_string_literal(b"max-content");
            }
            __v if __v == (YGUnit_YGUnitFitContent as i32) => {
                return Ptr::<i8>::from_string_literal(b"fit-content");
            }
            __v if __v == (YGUnit_YGUnitStretch as i32) => {
                return Ptr::<i8>::from_string_literal(b"stretch");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub fn YGWrapToString_170(mut value: YGWrap) -> Ptr<i8> {
    'switch: {
        match { (value as i32) } {
            __v if __v == (YGWrap_YGWrapNoWrap as i32) => {
                return Ptr::<i8>::from_string_literal(b"no-wrap");
            }
            __v if __v == (YGWrap_YGWrapWrap as i32) => {
                return Ptr::<i8>::from_string_literal(b"wrap");
            }
            __v if __v == (YGWrap_YGWrapWrapReverse as i32) => {
                return Ptr::<i8>::from_string_literal(b"wrap-reverse");
            }
            _ => {}
        }
    };
    return Ptr::<i8>::from_string_literal(b"unknown");
}
pub type facebook_yoga_LayoutType = i32;
pub const facebook_yoga_LayoutType_kLayout: facebook_yoga_LayoutType = 0;
pub const facebook_yoga_LayoutType_kMeasure: facebook_yoga_LayoutType = 1;
pub const facebook_yoga_LayoutType_kCachedLayout: facebook_yoga_LayoutType = 2;
pub const facebook_yoga_LayoutType_kCachedMeasure: facebook_yoga_LayoutType = 3;
pub type facebook_yoga_LayoutPassReason = i32;
pub const facebook_yoga_LayoutPassReason_kInitial: facebook_yoga_LayoutPassReason = 0;
pub const facebook_yoga_LayoutPassReason_kAbsLayout: facebook_yoga_LayoutPassReason = 1;
pub const facebook_yoga_LayoutPassReason_kStretch: facebook_yoga_LayoutPassReason = 2;
pub const facebook_yoga_LayoutPassReason_kMultilineStretch: facebook_yoga_LayoutPassReason = 3;
pub const facebook_yoga_LayoutPassReason_kFlexLayout: facebook_yoga_LayoutPassReason = 4;
pub const facebook_yoga_LayoutPassReason_kMeasureChild: facebook_yoga_LayoutPassReason = 5;
pub const facebook_yoga_LayoutPassReason_kAbsMeasureChild: facebook_yoga_LayoutPassReason = 6;
pub const facebook_yoga_LayoutPassReason_kFlexMeasure: facebook_yoga_LayoutPassReason = 7;
pub const facebook_yoga_LayoutPassReason_kGridLayout: facebook_yoga_LayoutPassReason = 8;
pub const facebook_yoga_LayoutPassReason_COUNT: facebook_yoga_LayoutPassReason = 9;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(60)]
pub struct facebook_yoga_LayoutData {
    #[offset(0)]
    pub layouts: i32,
    #[offset(4)]
    pub measures: i32,
    #[offset(8)]
    pub maxMeasureCache: u32,
    #[offset(12)]
    pub cachedLayouts: i32,
    #[offset(16)]
    pub cachedMeasures: i32,
    #[offset(20)]
    pub measureCallbacks: i32,
    #[offset(24)]
    #[byte_size(36)]
    pub measureCallbackReasonsCount: Vec<i32>,
}
impl Default for facebook_yoga_LayoutData {
    fn default() -> Self {
        {
            facebook_yoga_LayoutData {
                layouts: 0,
                measures: 0,
                maxMeasureCache: 0_u32,
                cachedLayouts: 0,
                cachedMeasures: 0,
                measureCallbacks: 0,
                measureCallbackReasonsCount: std::array::from_fn::<_, 9, _>(|_| 0_i32).to_vec(),
            }
        }
    }
}
pub type facebook_yoga_Event_Type = u32;
pub const facebook_yoga_Event_Type_NodeAllocation: facebook_yoga_Event_Type = 0;
pub const facebook_yoga_Event_Type_NodeDeallocation: facebook_yoga_Event_Type = 1;
pub const facebook_yoga_Event_Type_NodeLayout: facebook_yoga_Event_Type = 2;
pub const facebook_yoga_Event_Type_LayoutPassStart: facebook_yoga_Event_Type = 3;
pub const facebook_yoga_Event_Type_LayoutPassEnd: facebook_yoga_Event_Type = 4;
pub const facebook_yoga_Event_Type_MeasureCallbackStart: facebook_yoga_Event_Type = 5;
pub const facebook_yoga_Event_Type_MeasureCallbackEnd: facebook_yoga_Event_Type = 6;
pub const facebook_yoga_Event_Type_NodeBaselineStart: facebook_yoga_Event_Type = 7;
pub const facebook_yoga_Event_Type_NodeBaselineEnd: facebook_yoga_Event_Type = 8;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
    #[offset(0)]
    #[byte_size(8)]
    pub config: Ptr<YGConfig>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
    #[offset(0)]
    #[byte_size(8)]
    pub config: Ptr<YGConfig>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ {
    #[offset(0)]
    #[byte_size(8)]
    pub layoutData: Ptr<facebook_yoga_LayoutData>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(28)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_ {
    #[offset(0)]
    pub width: f32,
    #[offset(4)]
    pub widthMeasureMode: YGMeasureMode,
    #[offset(8)]
    pub height: f32,
    #[offset(12)]
    pub heightMeasureMode: YGMeasureMode,
    #[offset(16)]
    pub measuredWidth: f32,
    #[offset(20)]
    pub measuredHeight: f32,
    #[offset(24)]
    #[byte_size(4)]
    pub reason: facebook_yoga_LayoutPassReason,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_ {
    #[offset(0)]
    #[byte_size(4)]
    pub layoutType: facebook_yoga_LayoutType,
}
impl facebook_yoga_Event_Data {
    pub fn new_1(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
impl facebook_yoga_Event_Data {
    pub fn new_2(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct facebook_yoga_Event_Data {
    #[offset(0)]
    #[byte_size(8)]
    data_: AnyPtr,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct facebook_yoga_Event {}
impl facebook_yoga_Event {
    pub fn publish_3_0(
        mut node: Ptr<YGNode>,
        eventData: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_>,
    ) {
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_1({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((0_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
    pub fn publish_3_1(
        mut node: Ptr<YGNode>,
        eventData: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_>,
    ) {
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_2({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((1_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
}
pub fn YGNodeNew_6() -> Ptr<YGNode> {
    return ({ YGNodeNewWithConfig_200(({ YGConfigGetDefault_185() })) });
}
pub fn YGNodeNewWithConfig_200(mut config: Ptr<YGConfig>) -> Ptr<YGNode> {
    let mut node: Ptr<facebook_yoga_Node> = Ptr::alloc(facebook_yoga_Node::new_1({
        ({ resolveRef_73((config).clone()) })
    }));
    ({
        assertFatal_201(
            !((config).is_null()),
            Ptr::<i8>::from_string_literal(b"Tried to construct YGNode with null config"),
        )
    });
    ({
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
                    config: (config).clone(),
                },
            ));
        facebook_yoga_Event::publish_3_0(node.reinterpret_cast::<YGNode>(), _eventData.as_pointer())
    });
    return node.reinterpret_cast::<YGNode>();
}
pub fn YGNodeClone_202(mut oldNodeRef: Ptr<YGNode>) -> Ptr<YGNode> {
    let mut oldNode: Ptr<facebook_yoga_Node> = ({ resolveRef_181((oldNodeRef).clone()) });
    let mut node: Ptr<facebook_yoga_Node> = Ptr::alloc((*oldNode.upgrade().deref()).clone());
    ({
        let _node: Ptr<YGNode> = node.reinterpret_cast::<YGNode>();
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
                    config: ({ facebook_yoga_NodeImpl::getConfig(&node) })
                        .reinterpret_cast::<YGConfig>(),
                },
            ));
        facebook_yoga_Event::publish_3_0(_node, _eventData.as_pointer())
    });
    ({ facebook_yoga_NodeImpl::setOwner(&node, Ptr::<facebook_yoga_Node>::null()) });
    return node.reinterpret_cast::<YGNode>();
}
pub fn YGNodeFree_203(mut nodeRef: Ptr<YGNode>) {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_180((nodeRef).clone()) });
    {
        let mut owner: Ptr<facebook_yoga_Node> = ({ facebook_yoga_NodeImpl::getOwner(&node) });
        if !(owner).is_null() {
            ({ facebook_yoga_NodeImpl::removeChild_78(&owner, (node).clone()) });
            ({ facebook_yoga_NodeImpl::setOwner(&node, Ptr::<facebook_yoga_Node>::null()) });
            ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
        }
    }
    let mut childCount: usize = ({ facebook_yoga_NodeImpl::getChildCount(&node) });
    let mut i: usize = 0_usize;
    'loop_: while (i < childCount) {
        let mut child: Ptr<facebook_yoga_Node> = ({ facebook_yoga_NodeImpl::getChild(&node, i) });
        ({ facebook_yoga_NodeImpl::setOwner(&child, Ptr::<facebook_yoga_Node>::null()) });
        i.postfix_inc();
    }
    ({ facebook_yoga_NodeImpl::clearChildren(&node) });
    ({
        let _node: Ptr<YGNode> = node.reinterpret_cast::<YGNode>();
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                    config: ({ YGNodeGetConfig_204(node.reinterpret_cast::<YGNode>()) }),
                },
            ));
        facebook_yoga_Event::publish_3_1(_node, _eventData.as_pointer())
    });
    ({ resolveRef_180(node.reinterpret_cast::<YGNode>()) }).delete();
}
pub fn YGNodeFreeRecursive_13(mut rootRef: Ptr<YGNode>) {
    let mut root: Ptr<facebook_yoga_Node> = ({ resolveRef_180((rootRef).clone()) });
    let mut skipped: usize = 0_usize;
    'loop_: while ({ ({ facebook_yoga_NodeImpl::getChildCount(&root) }) } > { skipped }) {
        let mut child: Ptr<facebook_yoga_Node> =
            ({ facebook_yoga_NodeImpl::getChild(&root, skipped) });
        if ({ ({ facebook_yoga_NodeImpl::getOwner(&child) }) } != { (root).clone() }) {
            skipped = { (skipped).wrapping_add(1_usize) };
        } else {
            ({
                YGNodeRemoveChild_205(
                    root.reinterpret_cast::<YGNode>(),
                    child.reinterpret_cast::<YGNode>(),
                )
            });
            ({ YGNodeFreeRecursive_13(child.reinterpret_cast::<YGNode>()) });
        }
    }
    ({ YGNodeFree_203(root.reinterpret_cast::<YGNode>()) });
}
pub fn YGNodeFinalize_206(mut node: Ptr<YGNode>) {
    ({
        let _node: Ptr<YGNode> = (node).clone();
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                    config: ({ YGNodeGetConfig_204((node).clone()) }),
                },
            ));
        facebook_yoga_Event::publish_3_1(_node, _eventData.as_pointer())
    });
    ({ resolveRef_180((node).clone()) }).delete();
}
pub fn YGNodeReset_207(mut node: Ptr<YGNode>) {
    ({ facebook_yoga_NodeImpl::reset(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeCalculateLayout_12(
    mut node: Ptr<YGNode>,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut ownerDirection: YGDirection,
) {
    ({
        calculateLayout_208(
            ({ resolveRef_180((node).clone()) }),
            ownerWidth,
            ownerHeight,
            ({ scopedEnum_84(ownerDirection) }),
        )
    });
}
pub fn YGNodeGetHasNewLayout_209(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::getHasNewLayout(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetHasNewLayout_210(mut node: Ptr<YGNode>, mut hasNewLayout: bool) {
    ({
        facebook_yoga_NodeImpl::setHasNewLayout(&({ resolveRef_180((node).clone()) }), hasNewLayout)
    });
}
pub fn YGNodeIsDirty_211(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::isDirty(&({ resolveRef_181(node) })) });
}
pub fn YGNodeMarkDirty_212(mut nodeRef: Ptr<YGNode>) {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_180((nodeRef).clone()) });
    ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _condition: bool = ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) });
        assertFatalWithNode_213 ( _node , _condition , Ptr::<i8 >::from_string_literal(b"Only leaf nodes with custom measure functions should manually mark themselves as dirty" )  , )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&node) });
}
pub fn YGNodeSetDirtiedFunc_214(mut node: Ptr<YGNode>, mut dirtiedFunc: FnPtr<fn(Ptr<YGNode>)>) {
    ({
        facebook_yoga_NodeImpl::setDirtiedFunc(
            &({ resolveRef_180((node).clone()) }),
            (dirtiedFunc).clone(),
        )
    });
}
pub fn YGNodeGetDirtiedFunc_215(mut node: Ptr<YGNode>) -> FnPtr<fn(Ptr<YGNode>)> {
    return ({ facebook_yoga_NodeImpl::getDirtiedFunc(&({ resolveRef_181(node) })) });
}
pub fn YGNodeInsertChild_11(
    mut ownerRef: Ptr<YGNode>,
    mut childRef: Ptr<YGNode>,
    mut index: usize,
) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_180((ownerRef).clone()) });
    let mut child: Ptr<facebook_yoga_Node> = ({ resolveRef_180((childRef).clone()) });
    ({
        assertFatalWithNode_213(
            (owner).clone(),
            ({ facebook_yoga_NodeImpl::getOwner(&child) }).is_null(),
            Ptr::<i8>::from_string_literal(b"Child already has a owner, it must be removed first."),
        )
    });
    ({
        let _node: Ptr<facebook_yoga_Node> = (owner).clone();
        let _condition: bool = !({ facebook_yoga_NodeImpl::hasMeasureFunc(&owner) });
        assertFatalWithNode_213(
            _node,
            _condition,
            Ptr::<i8>::from_string_literal(
                b"Cannot add child: Nodes with measure functions cannot have children.",
            ),
        )
    });
    ({ facebook_yoga_NodeImpl::insertChild(&owner, (child).clone(), index) });
    ({ facebook_yoga_NodeImpl::setOwner(&child, (owner).clone()) });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
}
pub fn YGNodeSwapChild_216(mut ownerRef: Ptr<YGNode>, mut childRef: Ptr<YGNode>, mut index: usize) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_180((ownerRef).clone()) });
    let mut child: Ptr<facebook_yoga_Node> = ({ resolveRef_180((childRef).clone()) });
    ({ facebook_yoga_NodeImpl::replaceChild_76(&owner, (child).clone(), index) });
    ({ facebook_yoga_NodeImpl::setOwner(&child, (owner).clone()) });
}
pub fn YGNodeRemoveChild_205(mut ownerRef: Ptr<YGNode>, mut excludedChildRef: Ptr<YGNode>) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_180((ownerRef).clone()) });
    let mut excludedChild: Ptr<facebook_yoga_Node> =
        ({ resolveRef_180((excludedChildRef).clone()) });
    if (({ facebook_yoga_NodeImpl::getChildCount(&owner) }) == 0_usize) {
        return;
    }
    let mut childOwner: Ptr<facebook_yoga_Node> =
        ({ facebook_yoga_NodeImpl::getOwner(&excludedChild) });
    if ({ facebook_yoga_NodeImpl::removeChild_78(&owner, (excludedChild).clone()) }) {
        if ({ (owner).clone() } == { (childOwner).clone() }) {
            ({
                let _layout: Value<facebook_yoga_LayoutResults> =
                    Rc::new(RefCell::new(<facebook_yoga_LayoutResults>::default()));
                facebook_yoga_NodeImpl::setLayout(&excludedChild, _layout.as_pointer())
            });
            ({
                facebook_yoga_NodeImpl::setOwner(&excludedChild, Ptr::<facebook_yoga_Node>::null())
            });
            let mut dirtiedFunc: FnPtr<fn(Ptr<YGNode>)> =
                ({ facebook_yoga_NodeImpl::getDirtiedFunc(&excludedChild) });
            ({
                facebook_yoga_NodeImpl::setDirtiedFunc(
                    &excludedChild,
                    FnPtr::<fn(Ptr<YGNode>)>::null(),
                )
            });
            ({ facebook_yoga_NodeImpl::setDirty(&excludedChild, true) });
            ({ facebook_yoga_NodeImpl::setDirtiedFunc(&excludedChild, (dirtiedFunc).clone()) });
        }
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
    }
}
pub fn YGNodeRemoveAllChildren_217(mut ownerRef: Ptr<YGNode>) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_180((ownerRef).clone()) });
    let mut childCount: usize = ({ facebook_yoga_NodeImpl::getChildCount(&owner) });
    if (childCount == 0_usize) {
        return;
    }
    let mut firstChild: Ptr<facebook_yoga_Node> =
        ({ facebook_yoga_NodeImpl::getChild(&owner, 0_usize) });
    if ({ ({ facebook_yoga_NodeImpl::getOwner(&firstChild) }) } == { (owner).clone() }) {
        let mut i: usize = 0_usize;
        'loop_: while (i < childCount) {
            let mut oldChild: Ptr<facebook_yoga_Node> =
                ({ facebook_yoga_NodeImpl::getChild(&owner, i) });
            ({
                let _layout: Value<facebook_yoga_LayoutResults> =
                    Rc::new(RefCell::new(<facebook_yoga_LayoutResults>::default()));
                facebook_yoga_NodeImpl::setLayout(&oldChild, _layout.as_pointer())
            });
            ({ facebook_yoga_NodeImpl::setOwner(&oldChild, Ptr::<facebook_yoga_Node>::null()) });
            let mut dirtiedFunc: FnPtr<fn(Ptr<YGNode>)> =
                ({ facebook_yoga_NodeImpl::getDirtiedFunc(&oldChild) });
            ({
                facebook_yoga_NodeImpl::setDirtiedFunc(&oldChild, FnPtr::<fn(Ptr<YGNode>)>::null())
            });
            ({ facebook_yoga_NodeImpl::setDirty(&oldChild, true) });
            ({ facebook_yoga_NodeImpl::setDirtiedFunc(&oldChild, (dirtiedFunc).clone()) });
            i.postfix_inc();
        }
        ({ facebook_yoga_NodeImpl::clearChildren(&owner) });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
        return;
    }
    ({
        let _children: Value<Vec<Ptr<facebook_yoga_Node>>> = Rc::new(RefCell::new(Vec::new()));
        facebook_yoga_NodeImpl::setChildren(&owner, _children.as_pointer())
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
}
pub fn YGNodeSetChildren_218(
    mut ownerRef: Ptr<YGNode>,
    mut childrenRefs: Ptr<Ptr<YGNode>>,
    mut count: usize,
) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_180((ownerRef).clone()) });
    let mut children: Ptr<Ptr<facebook_yoga_Node>> =
        childrenRefs.reinterpret_cast::<Ptr<facebook_yoga_Node>>();
    if (owner).is_null() {
        return;
    }
    let childrenVector: Value<Vec<Ptr<facebook_yoga_Node>>> = Rc::new(RefCell::new({
        let mut __v = Vec::new();
        let mut __it = (children).clone();
        let __last = children.offset((count) as isize);
        while __it != __last {
            __v.push(__it.iter_deref().read());
            __it.iter_inc();
        }
        __v
    }));
    if (*childrenVector.borrow()).is_empty() {
        if (({ facebook_yoga_NodeImpl::getChildCount(&owner) }) > 0_usize) {
            let __range3: Ptr<Vec<Ptr<facebook_yoga_Node>>> =
                ({ facebook_yoga_NodeImpl::getChildren(&owner) });
            'loop_: for child in IterRange::new(__range3.decay(), __range3.decay().to_end()) {
                let mut child: Ptr<facebook_yoga_Node> = (child.read());
                ({
                    let _layout: Value<facebook_yoga_LayoutResults> =
                        Rc::new(RefCell::new(<facebook_yoga_LayoutResults>::default()));
                    facebook_yoga_NodeImpl::setLayout(&child, _layout.as_pointer())
                });
                ({ facebook_yoga_NodeImpl::setOwner(&child, Ptr::<facebook_yoga_Node>::null()) });
            }
            ({
                let _children: Value<Vec<Ptr<facebook_yoga_Node>>> =
                    Rc::new(RefCell::new(Vec::new()));
                facebook_yoga_NodeImpl::setChildren(&owner, _children.as_pointer())
            });
            ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
        }
    } else {
        if (({ facebook_yoga_NodeImpl::getChildCount(&owner) }) > 0_usize) {
            let __range3: Ptr<Vec<Ptr<facebook_yoga_Node>>> =
                ({ facebook_yoga_NodeImpl::getChildren(&owner) });
            'loop_: for oldChild in IterRange::new(__range3.decay(), __range3.decay().to_end()) {
                let mut oldChild: Ptr<facebook_yoga_Node> = (oldChild.read());
                if {
                    let mut __it = childrenVector.decay();
                    let __last = childrenVector.decay().to_end();
                    let __value = (oldChild).clone();
                    while __it != __last && __it.iter_deref().read() != __value {
                        __it.iter_inc();
                    }
                    __it
                } == childrenVector.decay().to_end()
                {
                    ({
                        let _layout: Value<facebook_yoga_LayoutResults> =
                            Rc::new(RefCell::new(<facebook_yoga_LayoutResults>::default()));
                        facebook_yoga_NodeImpl::setLayout(&oldChild, _layout.as_pointer())
                    });
                    ({
                        facebook_yoga_NodeImpl::setOwner(
                            &oldChild,
                            Ptr::<facebook_yoga_Node>::null(),
                        )
                    });
                }
            }
        }
        ({ facebook_yoga_NodeImpl::setChildren(&owner, childrenVector.as_pointer()) });
        'loop_: for child in IterRange::new(childrenVector.decay(), childrenVector.decay().to_end())
        {
            let mut child: Ptr<facebook_yoga_Node> = (child.read());
            ({ facebook_yoga_NodeImpl::setOwner(&child, (owner).clone()) });
        }
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
    }
}
pub fn YGNodeGetChild_219(mut nodeRef: Ptr<YGNode>, mut index: usize) -> Ptr<YGNode> {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_180((nodeRef).clone()) });
    if ({ index } < {
        (*({ facebook_yoga_NodeImpl::getChildren(&node) })
            .upgrade()
            .deref())
        .len()
    }) {
        return ({ facebook_yoga_NodeImpl::getChild(&node, index) }).reinterpret_cast::<YGNode>();
    }
    return Ptr::<YGNode>::null();
}
pub fn YGNodeGetChildCount_220(mut node: Ptr<YGNode>) -> usize {
    return (*({ facebook_yoga_NodeImpl::getChildren(&({ resolveRef_181((node).clone()) })) })
        .upgrade()
        .deref())
    .len();
}
pub fn YGNodeGetOwner_221(mut node: Ptr<YGNode>) -> Ptr<YGNode> {
    return ({ facebook_yoga_NodeImpl::getOwner(&({ resolveRef_180(node) })) })
        .reinterpret_cast::<YGNode>();
}
pub fn YGNodeGetParent_222(mut node: Ptr<YGNode>) -> Ptr<YGNode> {
    return ({ facebook_yoga_NodeImpl::getOwner(&({ resolveRef_180(node) })) })
        .reinterpret_cast::<YGNode>();
}
pub fn YGNodeSetConfig_223(mut node: Ptr<YGNode>, mut config: Ptr<YGConfig>) {
    ({
        facebook_yoga_NodeImpl::setConfig(
            &({ resolveRef_180((node).clone()) }),
            ({ resolveRef_72((config).clone()) }),
        )
    });
}
pub fn YGNodeGetConfig_204(mut node: Ptr<YGNode>) -> Ptr<YGConfig> {
    return ({ facebook_yoga_NodeImpl::getConfig(&({ resolveRef_180(node) })) })
        .reinterpret_cast::<YGConfig>();
}
pub fn YGNodeSetContext_224(mut node: Ptr<YGNode>, mut context: AnyPtr) {
    ({ facebook_yoga_NodeImpl::setContext(&({ resolveRef_180(node) }), context) });
    return;
}
pub fn YGNodeGetContext_225(mut node: Ptr<YGNode>) -> AnyPtr {
    return ({ facebook_yoga_NodeImpl::getContext(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetMeasureFunc_9(
    mut node: Ptr<YGNode>,
    mut measureFunc: FnPtr<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
) {
    ({
        facebook_yoga_NodeImpl::setMeasureFunc(
            &({ resolveRef_180((node).clone()) }),
            (measureFunc).clone(),
        )
    });
}
pub fn YGNodeHasMeasureFunc_226(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::hasMeasureFunc(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetMinContentMeasureFunc_227(
    mut node: Ptr<YGNode>,
    mut minContentMeasureFunc: FnPtr<
        fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
    >,
) {
    ({
        facebook_yoga_NodeImpl::setMinContentMeasureFunc(
            &({ resolveRef_180((node).clone()) }),
            (minContentMeasureFunc).clone(),
        )
    });
}
pub fn YGNodeHasMinContentMeasureFunc_228(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::hasMinContentMeasureFunc(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetMinContentWidth_229(mut node: Ptr<YGNode>, mut minContentWidth: f32) {
    ({
        facebook_yoga_NodeImpl::setMinContentWidth(
            &({ resolveRef_180((node).clone()) }),
            facebook_yoga_FloatOptional::new_1({ minContentWidth }),
        )
    });
}
pub fn YGNodeSetMinContentHeight_230(mut node: Ptr<YGNode>, mut minContentHeight: f32) {
    ({
        facebook_yoga_NodeImpl::setMinContentHeight(
            &({ resolveRef_180((node).clone()) }),
            facebook_yoga_FloatOptional::new_1({ minContentHeight }),
        )
    });
}
pub fn YGNodeGetMinContentWidth_231(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_NodeImpl::getMinContentWidth(&({ resolveRef_181(node) })) }),
            ))
            .as_pointer(),
            YGUndefined_23.with(|rc| *rc.borrow()),
        )
    });
}
pub fn YGNodeGetMinContentHeight_232(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_NodeImpl::getMinContentHeight(&({ resolveRef_181(node) })) }),
            ))
            .as_pointer(),
            YGUndefined_23.with(|rc| *rc.borrow()),
        )
    });
}
pub fn YGNodeSetBaselineFunc_233(
    mut node: Ptr<YGNode>,
    mut baselineFunc: FnPtr<fn(Ptr<YGNode>, f32, f32) -> f32>,
) {
    ({
        facebook_yoga_NodeImpl::setBaselineFunc(
            &({ resolveRef_180((node).clone()) }),
            (baselineFunc).clone(),
        )
    });
}
pub fn YGNodeHasBaselineFunc_234(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::hasBaselineFunc(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetIsReferenceBaseline_235(mut nodeRef: Ptr<YGNode>, mut isReferenceBaseline: bool) {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_180((nodeRef).clone()) });
    if ({ (({ facebook_yoga_NodeImpl::isReferenceBaseline(&node) }) as i32) } != {
        (isReferenceBaseline as i32)
    }) {
        ({ facebook_yoga_NodeImpl::setIsReferenceBaseline(&node, isReferenceBaseline) });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&node) });
    }
}
pub fn YGNodeIsReferenceBaseline_236(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::isReferenceBaseline(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetNodeType_237(mut node: Ptr<YGNode>, mut nodeType: YGNodeType) {
    ({
        facebook_yoga_NodeImpl::setNodeType(
            &({ resolveRef_180(node) }),
            ({ scopedEnum_98(nodeType) }),
        )
    });
    return;
}
pub fn YGNodeGetNodeType_238(mut node: Ptr<YGNode>) -> YGNodeType {
    return ({
        unscopedEnum_99(({ facebook_yoga_NodeImpl::getNodeType(&({ resolveRef_181(node) })) }))
    });
}
pub fn YGNodeSetAlwaysFormsContainingBlock_239(
    mut node: Ptr<YGNode>,
    mut alwaysFormsContainingBlock: bool,
) {
    ({
        facebook_yoga_NodeImpl::setAlwaysFormsContainingBlock(
            &({ resolveRef_180((node).clone()) }),
            alwaysFormsContainingBlock,
        )
    });
}
pub fn YGNodeGetAlwaysFormsContainingBlock_240(mut node: Ptr<YGNode>) -> bool {
    return ({ facebook_yoga_NodeImpl::alwaysFormsContainingBlock(&({ resolveRef_181(node) })) });
}
pub fn YGNodeCanUseCachedMeasurement_241(
    mut widthMode: YGMeasureMode,
    mut availableWidth: f32,
    mut heightMode: YGMeasureMode,
    mut availableHeight: f32,
    mut lastWidthMode: YGMeasureMode,
    mut lastAvailableWidth: f32,
    mut lastHeightMode: YGMeasureMode,
    mut lastAvailableHeight: f32,
    mut lastComputedWidth: f32,
    mut lastComputedHeight: f32,
    mut marginRow: f32,
    mut marginColumn: f32,
    mut config: Ptr<YGConfig>,
) -> bool {
    return ({
        let _widthMode: facebook_yoga_SizingMode =
            ({ sizingMode_104(({ scopedEnum_94(widthMode) })) });
        let _heightMode: facebook_yoga_SizingMode =
            ({ sizingMode_104(({ scopedEnum_94(heightMode) })) });
        let _lastWidthMode: facebook_yoga_SizingMode =
            ({ sizingMode_104(({ scopedEnum_94(lastWidthMode) })) });
        let _lastHeightMode: facebook_yoga_SizingMode =
            ({ sizingMode_104(({ scopedEnum_94(lastHeightMode) })) });
        canUseCachedMeasurement_242(
            _widthMode,
            availableWidth,
            _heightMode,
            availableHeight,
            _lastWidthMode,
            lastAvailableWidth,
            _lastHeightMode,
            lastAvailableHeight,
            lastComputedWidth,
            lastComputedHeight,
            marginRow,
            marginColumn,
            ({ resolveRef_72(config) }),
        )
    });
}
pub fn getResolvedLayoutProperty_243(
    mut nodeRef: Ptr<YGNode>,
    mut edge: facebook_yoga_Edge,
) -> f32 {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    ({
        assertFatalWithNode_213(
            (node).clone(),
            (edge <= facebook_yoga_Edge_End),
            Ptr::<i8>::from_string_literal(
                b"Cannot get layout properties of multi-edge shorthands",
            ),
        )
    });
    if (edge == facebook_yoga_Edge_Start) {
        if (({
            facebook_yoga_LayoutResultsImpl::direction(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            )
        }) == facebook_yoga_Direction_RTL)
        {
            return ({
                facebook_yoga_LayoutResultsImpl::margin(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        } else {
            return ({
                facebook_yoga_LayoutResultsImpl::margin(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        }
    }
    if (edge == facebook_yoga_Edge_End) {
        if (({
            facebook_yoga_LayoutResultsImpl::direction(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            )
        }) == facebook_yoga_Direction_RTL)
        {
            return ({
                facebook_yoga_LayoutResultsImpl::margin(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        } else {
            return ({
                facebook_yoga_LayoutResultsImpl::margin(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        }
    }
    return ({
        facebook_yoga_LayoutResultsImpl::margin(
            &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            (edge as facebook_yoga_PhysicalEdge),
        )
    });
}
pub fn getResolvedLayoutProperty_244(
    mut nodeRef: Ptr<YGNode>,
    mut edge: facebook_yoga_Edge,
) -> f32 {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    ({
        assertFatalWithNode_213(
            (node).clone(),
            (edge <= facebook_yoga_Edge_End),
            Ptr::<i8>::from_string_literal(
                b"Cannot get layout properties of multi-edge shorthands",
            ),
        )
    });
    if (edge == facebook_yoga_Edge_Start) {
        if (({
            facebook_yoga_LayoutResultsImpl::direction(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            )
        }) == facebook_yoga_Direction_RTL)
        {
            return ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        } else {
            return ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        }
    }
    if (edge == facebook_yoga_Edge_End) {
        if (({
            facebook_yoga_LayoutResultsImpl::direction(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            )
        }) == facebook_yoga_Direction_RTL)
        {
            return ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        } else {
            return ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        }
    }
    return ({
        facebook_yoga_LayoutResultsImpl::border(
            &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            (edge as facebook_yoga_PhysicalEdge),
        )
    });
}
pub fn getResolvedLayoutProperty_245(
    mut nodeRef: Ptr<YGNode>,
    mut edge: facebook_yoga_Edge,
) -> f32 {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    ({
        assertFatalWithNode_213(
            (node).clone(),
            (edge <= facebook_yoga_Edge_End),
            Ptr::<i8>::from_string_literal(
                b"Cannot get layout properties of multi-edge shorthands",
            ),
        )
    });
    if (edge == facebook_yoga_Edge_Start) {
        if (({
            facebook_yoga_LayoutResultsImpl::direction(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            )
        }) == facebook_yoga_Direction_RTL)
        {
            return ({
                facebook_yoga_LayoutResultsImpl::padding(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        } else {
            return ({
                facebook_yoga_LayoutResultsImpl::padding(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        }
    }
    if (edge == facebook_yoga_Edge_End) {
        if (({
            facebook_yoga_LayoutResultsImpl::direction(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            )
        }) == facebook_yoga_Direction_RTL)
        {
            return ({
                facebook_yoga_LayoutResultsImpl::padding(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
        } else {
            return ({
                facebook_yoga_LayoutResultsImpl::padding(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_PhysicalEdge_Right,
                )
            });
        }
    }
    return ({
        facebook_yoga_LayoutResultsImpl::padding(
            &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
            (edge as facebook_yoga_PhysicalEdge),
        )
    });
}
pub fn YGNodeLayoutGetLeft_246(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_PhysicalEdge_Left,
        )
    });
}
pub fn YGNodeLayoutGetTop_247(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_PhysicalEdge_Top,
        )
    });
}
pub fn YGNodeLayoutGetRight_248(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_PhysicalEdge_Right,
        )
    });
}
pub fn YGNodeLayoutGetBottom_249(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
}
pub fn YGNodeLayoutGetWidth_250(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub fn YGNodeLayoutGetHeight_251(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub fn YGNodeLayoutGetDirection_252(mut node: Ptr<YGNode>) -> YGDirection {
    return ({
        unscopedEnum_85(
            ({
                facebook_yoga_LayoutResultsImpl::direction(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeLayoutGetHadOverflow_253(mut node: Ptr<YGNode>) -> bool {
    return ({
        facebook_yoga_LayoutResultsImpl::hadOverflow(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
        )
    });
}
pub fn YGNodeLayoutGetMargin_254(mut node: Ptr<YGNode>, mut edge: YGEdge) -> f32 {
    return ({ getResolvedLayoutProperty_243(node, ({ scopedEnum_89(edge) })) });
}
pub fn YGNodeLayoutGetBorder_255(mut node: Ptr<YGNode>, mut edge: YGEdge) -> f32 {
    return ({ getResolvedLayoutProperty_244(node, ({ scopedEnum_89(edge) })) });
}
pub fn YGNodeLayoutGetPadding_256(mut node: Ptr<YGNode>, mut edge: YGEdge) -> f32 {
    return ({ getResolvedLayoutProperty_245(node, ({ scopedEnum_89(edge) })) });
}
pub fn YGNodeLayoutGetRawHeight_257(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::rawDimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub fn YGNodeLayoutGetRawWidth_258(mut node: Ptr<YGNode>) -> f32 {
    return ({
        facebook_yoga_LayoutResultsImpl::rawDimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_181(node) })) }),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub fn updateStyle_259(mut node: Ptr<YGNode>, mut value: facebook_yoga_Direction) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::direction(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Direction = value;
            facebook_yoga_StyleImpl::setDirection(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_260(mut node: Ptr<YGNode>, mut value: facebook_yoga_FlexDirection) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::flexDirection(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_FlexDirection = value;
            facebook_yoga_StyleImpl::setFlexDirection(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_261(mut node: Ptr<YGNode>, mut value: facebook_yoga_Justify) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::justifyContent(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_StyleImpl::setJustifyContent(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_262(mut node: Ptr<YGNode>, mut value: facebook_yoga_Justify) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::justifyItems(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_StyleImpl::setJustifyItems(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_263(mut node: Ptr<YGNode>, mut value: facebook_yoga_Justify) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::justifySelf(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_StyleImpl::setJustifySelf(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_264(mut node: Ptr<YGNode>, mut value: facebook_yoga_Align) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::alignContent(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Align = value;
            facebook_yoga_StyleImpl::setAlignContent(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_265(mut node: Ptr<YGNode>, mut value: facebook_yoga_Align) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::alignItems(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Align = value;
            facebook_yoga_StyleImpl::setAlignItems(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_266(mut node: Ptr<YGNode>, mut value: facebook_yoga_Align) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::alignSelf(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Align = value;
            facebook_yoga_StyleImpl::setAlignSelf(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_267(mut node: Ptr<YGNode>, mut value: facebook_yoga_PositionType) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::positionType(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_PositionType = value;
            facebook_yoga_StyleImpl::setPositionType(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_268(mut node: Ptr<YGNode>, mut value: facebook_yoga_Wrap) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::flexWrap(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Wrap = value;
            facebook_yoga_StyleImpl::setFlexWrap(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_269(mut node: Ptr<YGNode>, mut value: facebook_yoga_Overflow) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::overflow(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Overflow = value;
            facebook_yoga_StyleImpl::setOverflow(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_270(mut node: Ptr<YGNode>, mut value: facebook_yoga_Display) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::display(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Display = value;
            facebook_yoga_StyleImpl::setDisplay(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_271(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::flex(&style) });
        operator_eq_114(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setFlex(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_272(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::flexGrow(&style) });
        operator_eq_114(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setFlexGrow(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_273(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::flexShrink(&style) });
        operator_eq_114(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setFlexShrink(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_274(mut node: Ptr<YGNode>, value: facebook_yoga_StyleSizeLength) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleSizeLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::flexBasis(&style) }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _value: facebook_yoga_StyleSizeLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setFlexBasis(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_275(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::aspectRatio(&style) });
        operator_eq_114(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setAspectRatio(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_276(mut node: Ptr<YGNode>, mut value: facebook_yoga_BoxSizing) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::boxSizing(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_BoxSizing = value;
            facebook_yoga_StyleImpl::setBoxSizing(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_277(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_GridLineImpl::operator_eq(
            &({ facebook_yoga_StyleImpl::gridColumnStart(&style) }),
            value.as_pointer(),
        )
    }) {
        ({
            let _value: facebook_yoga_GridLine = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setGridColumnStart(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_278(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_GridLineImpl::operator_eq(
            &({ facebook_yoga_StyleImpl::gridColumnEnd(&style) }),
            value.as_pointer(),
        )
    }) {
        ({
            let _value: facebook_yoga_GridLine = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setGridColumnEnd(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_279(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_GridLineImpl::operator_eq(
            &({ facebook_yoga_StyleImpl::gridRowStart(&style) }),
            value.as_pointer(),
        )
    }) {
        ({
            let _value: facebook_yoga_GridLine = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setGridRowStart(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_280(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_GridLineImpl::operator_eq(
            &({ facebook_yoga_StyleImpl::gridRowEnd(&style) }),
            value.as_pointer(),
        )
    }) {
        ({
            let _value: facebook_yoga_GridLine = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setGridRowEnd(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_281(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = idx;
                    facebook_yoga_StyleImpl::position(&style, _edge)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setPosition(&style, _edge, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_282(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = idx;
                    facebook_yoga_StyleImpl::margin(&style, _edge)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setMargin(&style, _edge, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_283(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = idx;
                    facebook_yoga_StyleImpl::padding(&style, _edge)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setPadding(&style, _edge, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_284(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = idx;
                    facebook_yoga_StyleImpl::border(&style, _edge)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _edge: facebook_yoga_Edge = idx;
            let _value: facebook_yoga_StyleLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setBorder(&style, _edge, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_285(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Gutter,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _gutter: facebook_yoga_Gutter = idx;
                    facebook_yoga_StyleImpl::gap(&style, _gutter)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _gutter: facebook_yoga_Gutter = idx;
            let _value: facebook_yoga_StyleLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setGap(&style, _gutter, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_286(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Dimension,
    value: facebook_yoga_StyleSizeLength,
) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleSizeLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _axis: facebook_yoga_Dimension = idx;
                    facebook_yoga_StyleImpl::dimension(&style, _axis)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _axis: facebook_yoga_Dimension = idx;
            let _value: facebook_yoga_StyleSizeLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setDimension(&style, _axis, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_287(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Dimension,
    value: facebook_yoga_StyleSizeLength,
) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleSizeLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _axis: facebook_yoga_Dimension = idx;
                    facebook_yoga_StyleImpl::minDimension(&style, _axis)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _axis: facebook_yoga_Dimension = idx;
            let _value: facebook_yoga_StyleSizeLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setMinDimension(&style, _axis, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn updateStyle_288(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Dimension,
    value: facebook_yoga_StyleSizeLength,
) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) });
    if !({
        facebook_yoga_StyleSizeLengthImpl::operator_eq(
            &Rc::new(RefCell::new(
                ({
                    let _axis: facebook_yoga_Dimension = idx;
                    facebook_yoga_StyleImpl::maxDimension(&style, _axis)
                }),
            ))
            .as_pointer(),
            value.as_pointer(),
        )
    }) {
        ({
            let _axis: facebook_yoga_Dimension = idx;
            let _value: facebook_yoga_StyleSizeLength = (*value.borrow()).clone();
            facebook_yoga_StyleImpl::setMaxDimension(&style, _axis, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
    }
}
pub fn YGNodeCopyStyle_289(mut dstNode: Ptr<YGNode>, mut srcNode: Ptr<YGNode>) {
    let mut dst: Ptr<facebook_yoga_Node> = ({ resolveRef_180((dstNode).clone()) });
    let mut src: Ptr<facebook_yoga_Node> = ({ resolveRef_181((srcNode).clone()) });
    if !({
        facebook_yoga_StyleImpl::operator_eq(
            &({ facebook_yoga_NodeImpl::style_24(&dst) }),
            ({ facebook_yoga_NodeImpl::style_25(&src) }),
        )
    }) {
        ({ facebook_yoga_NodeImpl::setStyle(&dst, ({ facebook_yoga_NodeImpl::style_25(&src) })) });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&dst) });
    }
}
pub fn YGNodeStyleSetDirection_290(mut node: Ptr<YGNode>, mut value: YGDirection) {
    ({ updateStyle_259((node).clone(), ({ scopedEnum_84(value) })) });
}
pub fn YGNodeStyleGetDirection_291(mut node: Ptr<YGNode>) -> YGDirection {
    return ({
        unscopedEnum_85(
            ({
                facebook_yoga_StyleImpl::direction(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetFlexDirection_15(mut node: Ptr<YGNode>, mut flexDirection: YGFlexDirection) {
    ({ updateStyle_260((node).clone(), ({ scopedEnum_124(flexDirection) })) });
}
pub fn YGNodeStyleGetFlexDirection_292(mut node: Ptr<YGNode>) -> YGFlexDirection {
    return ({
        unscopedEnum_125(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetJustifyContent_293(mut node: Ptr<YGNode>, mut justifyContent: YGJustify) {
    ({ updateStyle_261((node).clone(), ({ scopedEnum_150(justifyContent) })) });
}
pub fn YGNodeStyleGetJustifyContent_294(mut node: Ptr<YGNode>) -> YGJustify {
    return ({
        unscopedEnum_151(
            ({
                facebook_yoga_StyleImpl::justifyContent(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetJustifyItems_295(mut node: Ptr<YGNode>, mut justifyItems: YGJustify) {
    ({ updateStyle_262((node).clone(), ({ scopedEnum_150(justifyItems) })) });
}
pub fn YGNodeStyleGetJustifyItems_296(mut node: Ptr<YGNode>) -> YGJustify {
    return ({
        unscopedEnum_151(
            ({
                facebook_yoga_StyleImpl::justifyItems(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetJustifySelf_297(mut node: Ptr<YGNode>, mut justifySelf: YGJustify) {
    ({ updateStyle_263((node).clone(), ({ scopedEnum_150(justifySelf) })) });
}
pub fn YGNodeStyleGetJustifySelf_298(mut node: Ptr<YGNode>) -> YGJustify {
    return ({
        unscopedEnum_151(
            ({
                facebook_yoga_StyleImpl::justifySelf(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetAlignContent_299(mut node: Ptr<YGNode>, mut alignContent: YGAlign) {
    ({ updateStyle_264((node).clone(), ({ scopedEnum_137(alignContent) })) });
}
pub fn YGNodeStyleGetAlignContent_300(mut node: Ptr<YGNode>) -> YGAlign {
    return ({
        unscopedEnum_138(
            ({
                facebook_yoga_StyleImpl::alignContent(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetAlignItems_301(mut node: Ptr<YGNode>, mut alignItems: YGAlign) {
    ({ updateStyle_265((node).clone(), ({ scopedEnum_137(alignItems) })) });
}
pub fn YGNodeStyleGetAlignItems_302(mut node: Ptr<YGNode>) -> YGAlign {
    return ({
        unscopedEnum_138(
            ({
                facebook_yoga_StyleImpl::alignItems(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetAlignSelf_303(mut node: Ptr<YGNode>, mut alignSelf: YGAlign) {
    ({ updateStyle_266((node).clone(), ({ scopedEnum_137(alignSelf) })) });
}
pub fn YGNodeStyleGetAlignSelf_304(mut node: Ptr<YGNode>) -> YGAlign {
    return ({
        unscopedEnum_138(
            ({
                facebook_yoga_StyleImpl::alignSelf(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetPositionType_305(mut node: Ptr<YGNode>, mut positionType: YGPositionType) {
    ({ updateStyle_267((node).clone(), ({ scopedEnum_158(positionType) })) });
}
pub fn YGNodeStyleGetPositionType_306(mut node: Ptr<YGNode>) -> YGPositionType {
    return ({
        unscopedEnum_159(
            ({
                facebook_yoga_StyleImpl::positionType(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetFlexWrap_307(mut node: Ptr<YGNode>, mut flexWrap: YGWrap) {
    ({ updateStyle_268((node).clone(), ({ scopedEnum_167(flexWrap) })) });
}
pub fn YGNodeStyleGetFlexWrap_308(mut node: Ptr<YGNode>) -> YGWrap {
    return ({
        unscopedEnum_168(
            ({
                facebook_yoga_StyleImpl::flexWrap(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetOverflow_309(mut node: Ptr<YGNode>, mut overflow: YGOverflow) {
    ({ updateStyle_269((node).clone(), ({ scopedEnum_154(overflow) })) });
}
pub fn YGNodeStyleGetOverflow_310(mut node: Ptr<YGNode>) -> YGOverflow {
    return ({
        unscopedEnum_155(
            ({
                facebook_yoga_StyleImpl::overflow(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetDisplay_311(mut node: Ptr<YGNode>, mut display: YGDisplay) {
    let mut wasContents: bool = (({
        facebook_yoga_StyleImpl::display(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
        )
    }) == facebook_yoga_Display_Contents);
    ({ updateStyle_270((node).clone(), ({ scopedEnum_74(display) })) });
    let mut owner: Ptr<facebook_yoga_Node> =
        ({ facebook_yoga_NodeImpl::getOwner(&({ resolveRef_180((node).clone()) })) });
    if (!((owner).is_null()))
        && ((wasContents as i32)
            != (((display as i32) == (YGDisplay_YGDisplayContents as i32)) as i32))
    {
        ({
            let _children: Ptr<Vec<Ptr<facebook_yoga_Node>>> =
                ({ facebook_yoga_NodeImpl::getChildren(&owner) });
            facebook_yoga_NodeImpl::setChildren(&owner, _children)
        });
    }
}
pub fn YGNodeStyleGetDisplay_312(mut node: Ptr<YGNode>) -> YGDisplay {
    return ({
        unscopedEnum_75(
            ({
                facebook_yoga_StyleImpl::display(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetFlex_10(mut node: Ptr<YGNode>, mut flex: f32) {
    ({ updateStyle_271((node).clone(), facebook_yoga_FloatOptional::new_1({ flex })) });
}
pub fn YGNodeStyleGetFlex_313(mut nodeRef: Ptr<YGNode>) -> f32 {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    return if ({
        facebook_yoga_FloatOptionalImpl::isUndefined(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::flex(&({ facebook_yoga_NodeImpl::style_25(&node) })) }),
            ))
            .as_pointer(),
        )
    }) {
        YGUndefined_23.with(|rc| *rc.borrow())
    } else {
        ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::flex(
                            &({ facebook_yoga_NodeImpl::style_25(&node) }),
                        )
                    }),
                ))
                .as_pointer(),
            )
        })
    };
}
pub fn YGNodeStyleSetFlexGrow_14(mut node: Ptr<YGNode>, mut flexGrow: f32) {
    ({
        updateStyle_272(
            (node).clone(),
            facebook_yoga_FloatOptional::new_1({ flexGrow }),
        )
    });
}
pub fn YGNodeStyleGetFlexGrow_314(mut nodeRef: Ptr<YGNode>) -> f32 {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    return if ({
        facebook_yoga_FloatOptionalImpl::isUndefined(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::flexGrow(
                        &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    }) {
        DefaultFlexGrow_177.with(|rc| *rc.borrow())
    } else {
        ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::flexGrow(
                            &({ facebook_yoga_NodeImpl::style_25(&node) }),
                        )
                    }),
                ))
                .as_pointer(),
            )
        })
    };
}
pub fn YGNodeStyleSetFlexShrink_315(mut node: Ptr<YGNode>, mut flexShrink: f32) {
    ({
        updateStyle_273(
            (node).clone(),
            facebook_yoga_FloatOptional::new_1({ flexShrink }),
        )
    });
}
pub fn YGNodeStyleGetFlexShrink_316(mut nodeRef: Ptr<YGNode>) -> f32 {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    return if ({
        facebook_yoga_FloatOptionalImpl::isUndefined(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::flexShrink(
                        &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    }) {
        (if ({
            facebook_yoga_ConfigImpl::useWebDefaults(
                &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            )
        }) {
            WebDefaultFlexShrink_179.with(|rc| *rc.borrow())
        } else {
            DefaultFlexShrink_178.with(|rc| *rc.borrow())
        })
    } else {
        ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::flexShrink(
                            &({ facebook_yoga_NodeImpl::style_25(&node) }),
                        )
                    }),
                ))
                .as_pointer(),
            )
        })
    };
}
pub fn YGNodeStyleSetFlexBasis_317(mut node: Ptr<YGNode>, mut flexBasis: f32) {
    ({
        updateStyle_274(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::points(flexBasis) }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisPercent_318(mut node: Ptr<YGNode>, mut flexBasisPercent: f32) {
    ({
        updateStyle_274(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::percent(flexBasisPercent) }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisAuto_319(mut node: Ptr<YGNode>) {
    ({
        updateStyle_274(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisMaxContent_320(mut node: Ptr<YGNode>) {
    ({
        updateStyle_274(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisFitContent_321(mut node: Ptr<YGNode>) {
    ({
        updateStyle_274(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisStretch_322(mut node: Ptr<YGNode>) {
    ({
        updateStyle_274(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetFlexBasis_323(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::flexBasis(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetPosition_324(mut node: Ptr<YGNode>, mut edge: YGEdge, mut points: f32) {
    ({
        updateStyle_281(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetPositionPercent_325(
    mut node: Ptr<YGNode>,
    mut edge: YGEdge,
    mut percent: f32,
) {
    ({
        updateStyle_281(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetPositionAuto_326(mut node: Ptr<YGNode>, mut edge: YGEdge) {
    ({
        updateStyle_281(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleGetPosition_327(mut node: Ptr<YGNode>, mut edge: YGEdge) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::position(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        ({ scopedEnum_89(edge) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMargin_328(mut node: Ptr<YGNode>, mut edge: YGEdge, mut points: f32) {
    ({
        updateStyle_282(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetMarginPercent_329(mut node: Ptr<YGNode>, mut edge: YGEdge, mut percent: f32) {
    ({
        updateStyle_282(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetMarginAuto_330(mut node: Ptr<YGNode>, mut edge: YGEdge) {
    ({
        updateStyle_282(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleGetMargin_331(mut node: Ptr<YGNode>, mut edge: YGEdge) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::margin(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        ({ scopedEnum_89(edge) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetPadding_332(mut node: Ptr<YGNode>, mut edge: YGEdge, mut points: f32) {
    ({
        updateStyle_283(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetPaddingPercent_333(mut node: Ptr<YGNode>, mut edge: YGEdge, mut percent: f32) {
    ({
        updateStyle_283(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleGetPadding_334(mut node: Ptr<YGNode>, mut edge: YGEdge) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::padding(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        ({ scopedEnum_89(edge) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetBorder_335(mut node: Ptr<YGNode>, mut edge: YGEdge, mut border: f32) {
    ({
        updateStyle_284(
            (node).clone(),
            ({ scopedEnum_89(edge) }),
            ({ facebook_yoga_StyleLength::points(border) }),
        )
    });
}
pub fn YGNodeStyleGetBorder_336(mut node: Ptr<YGNode>, mut edge: YGEdge) -> f32 {
    let border: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
        ({
            facebook_yoga_StyleImpl::border(
                &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181((node).clone()) })) }),
                ({ scopedEnum_89(edge) }),
            )
        }),
    ));
    if ({ facebook_yoga_StyleLengthImpl::isUndefined(&border.as_pointer()) })
        || ({ facebook_yoga_StyleLengthImpl::isAuto(&border.as_pointer()) })
    {
        return YGUndefined_23.with(|rc| *rc.borrow());
    }
    return { ({ facebook_yoga_StyleLengthImpl::to_YGValue(&border.as_pointer()) }).value };
}
pub fn YGNodeStyleSetGap_337(mut node: Ptr<YGNode>, mut gutter: YGGutter, mut gapLength: f32) {
    ({
        updateStyle_285(
            (node).clone(),
            ({ scopedEnum_146(gutter) }),
            ({ facebook_yoga_StyleLength::points(gapLength) }),
        )
    });
}
pub fn YGNodeStyleSetGapPercent_338(mut node: Ptr<YGNode>, mut gutter: YGGutter, mut percent: f32) {
    ({
        updateStyle_285(
            (node).clone(),
            ({ scopedEnum_146(gutter) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleGetGap_339(mut node: Ptr<YGNode>, mut gutter: YGGutter) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::gap(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        ({ scopedEnum_146(gutter) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetAspectRatio_340(mut node: Ptr<YGNode>, mut aspectRatio: f32) {
    ({
        updateStyle_275(
            (node).clone(),
            facebook_yoga_FloatOptional::new_1({ aspectRatio }),
        )
    });
}
pub fn YGNodeStyleGetAspectRatio_341(mut node: Ptr<YGNode>) -> f32 {
    let op: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            facebook_yoga_StyleImpl::aspectRatio(
                &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181((node).clone()) })) }),
            )
        }),
    ));
    return if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&op.as_pointer()) }) {
        YGUndefined_23.with(|rc| *rc.borrow())
    } else {
        ({ facebook_yoga_FloatOptionalImpl::unwrap(&op.as_pointer()) })
    };
}
pub fn YGNodeStyleSetBoxSizing_342(mut node: Ptr<YGNode>, mut boxSizing: YGBoxSizing) {
    ({ updateStyle_276((node).clone(), ({ scopedEnum_141(boxSizing) })) });
}
pub fn YGNodeStyleGetBoxSizing_343(mut node: Ptr<YGNode>) -> YGBoxSizing {
    return ({
        unscopedEnum_142(
            ({
                facebook_yoga_StyleImpl::boxSizing(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetWidth_7(mut node: Ptr<YGNode>, mut points: f32) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetWidthPercent_344(mut node: Ptr<YGNode>, mut percent: f32) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetWidthAuto_345(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleSetWidthMaxContent_346(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetWidthFitContent_347(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetWidthStretch_348(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetWidth_349(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::dimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        facebook_yoga_Dimension_Width,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetHeight_8(mut node: Ptr<YGNode>, mut points: f32) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetHeightPercent_350(mut node: Ptr<YGNode>, mut percent: f32) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetHeightAuto_351(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleSetHeightMaxContent_352(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetHeightFitContent_353(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetHeightStretch_354(mut node: Ptr<YGNode>) {
    ({
        updateStyle_286(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetHeight_355(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::dimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        facebook_yoga_Dimension_Height,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMinWidth_356(mut node: Ptr<YGNode>, mut minWidth: f32) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::points(minWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthPercent_357(mut node: Ptr<YGNode>, mut minWidth: f32) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::percent(minWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthMaxContent_358(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthFitContent_359(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthStretch_360(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMinWidth_361(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::minDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        facebook_yoga_Dimension_Width,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMinHeight_362(mut node: Ptr<YGNode>, mut minHeight: f32) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::points(minHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightPercent_363(mut node: Ptr<YGNode>, mut minHeight: f32) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::percent(minHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightMaxContent_364(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightFitContent_365(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightStretch_366(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMinHeight_367(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::minDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        facebook_yoga_Dimension_Height,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMaxWidth_368(mut node: Ptr<YGNode>, mut maxWidth: f32) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::points(maxWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthPercent_369(mut node: Ptr<YGNode>, mut maxWidth: f32) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::percent(maxWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthMaxContent_370(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthFitContent_371(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthStretch_372(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMaxWidth_373(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::maxDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        facebook_yoga_Dimension_Width,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMaxHeight_374(mut node: Ptr<YGNode>, mut maxHeight: f32) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::points(maxHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightPercent_375(mut node: Ptr<YGNode>, mut maxHeight: f32) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::percent(maxHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightMaxContent_376(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightFitContent_377(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightStretch_378(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMaxHeight_379(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::maxDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181(node) })) }),
                        facebook_yoga_Dimension_Height,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetGridColumnStart_380(mut node: Ptr<YGNode>, mut gridColumnStart: i32) {
    ({
        updateStyle_277(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridColumnStart) }),
        )
    });
}
pub fn YGNodeStyleSetGridColumnStartAuto_381(mut node: Ptr<YGNode>) {
    ({ updateStyle_277((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridColumnStartSpan_382(mut node: Ptr<YGNode>, mut span: i32) {
    ({ updateStyle_277((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridColumnStart_383(mut node: Ptr<YGNode>) -> i32 {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridColumnStart(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0
    };
}
pub fn YGNodeStyleSetGridColumnEnd_384(mut node: Ptr<YGNode>, mut gridColumnEnd: i32) {
    ({
        updateStyle_278(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridColumnEnd) }),
        )
    });
}
pub fn YGNodeStyleSetGridColumnEndAuto_385(mut node: Ptr<YGNode>) {
    ({ updateStyle_278((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridColumnEndSpan_386(mut node: Ptr<YGNode>, mut span: i32) {
    ({ updateStyle_278((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridColumnEnd_387(mut node: Ptr<YGNode>) -> i32 {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridColumnEnd(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0
    };
}
pub fn YGNodeStyleSetGridRowStart_388(mut node: Ptr<YGNode>, mut gridRowStart: i32) {
    ({
        updateStyle_279(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridRowStart) }),
        )
    });
}
pub fn YGNodeStyleSetGridRowStartAuto_389(mut node: Ptr<YGNode>) {
    ({ updateStyle_279((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridRowStartSpan_390(mut node: Ptr<YGNode>, mut span: i32) {
    ({ updateStyle_279((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridRowStart_391(mut node: Ptr<YGNode>) -> i32 {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridRowStart(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0
    };
}
pub fn YGNodeStyleSetGridRowEnd_392(mut node: Ptr<YGNode>, mut gridRowEnd: i32) {
    ({
        updateStyle_280(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridRowEnd) }),
        )
    });
}
pub fn YGNodeStyleSetGridRowEndAuto_393(mut node: Ptr<YGNode>) {
    ({ updateStyle_280((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridRowEndSpan_394(mut node: Ptr<YGNode>, mut span: i32) {
    ({ updateStyle_280((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridRowEnd_395(mut node: Ptr<YGNode>) -> i32 {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridRowEnd(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_181((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0
    };
}
pub fn gridTrackSizeFromTypeAndValue_396(
    mut type_: YGGridTrackType,
    mut value: f32,
) -> facebook_yoga_GridTrackSize {
    'switch: {
        match { (type_ as i32) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as i32) => {
                return ({ facebook_yoga_GridTrackSize::length(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as i32) => {
                return ({ facebook_yoga_GridTrackSize::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as i32) => {
                return ({ facebook_yoga_GridTrackSize::fr(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as i32) => {
                return ({ facebook_yoga_GridTrackSize::auto_() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as i32) => {
                return ({ facebook_yoga_GridTrackSize::auto_() });
            }
            _ => {}
        }
    };
    ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Unknown YGGridTrackType")) });
    panic!("ub: non-void function does not return a value")
}
pub fn styleSizeLengthFromTypeAndValue_397(
    mut type_: YGGridTrackType,
    mut value: f32,
) -> facebook_yoga_StyleSizeLength {
    'switch: {
        match { (type_ as i32) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as i32) => {
                return ({ facebook_yoga_StyleSizeLength::points(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as i32) => {
                return ({ facebook_yoga_StyleSizeLength::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as i32) => {
                return ({ facebook_yoga_StyleSizeLength::stretch(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as i32) => {
                return ({ facebook_yoga_StyleSizeLength::ofAuto() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as i32) => {
                return ({ facebook_yoga_StyleSizeLength::ofAuto() });
            }
            _ => {}
        }
    };
    ({ fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Unknown YGGridTrackType")) });
    panic!("ub: non-void function does not return a value")
}
pub fn YGNodeStyleSetGridTemplateColumnsCount_398(mut node: Ptr<YGNode>, mut count: usize) {
    ({
        facebook_yoga_StyleImpl::resizeGridTemplateColumns(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateColumn_399(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateColumnMinMax_400(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateRowsCount_401(mut node: Ptr<YGNode>, mut count: usize) {
    ({
        facebook_yoga_StyleImpl::resizeGridTemplateRows(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateRow_402(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateRowMinMax_403(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoColumnsCount_404(mut node: Ptr<YGNode>, mut count: usize) {
    ({
        facebook_yoga_StyleImpl::resizeGridAutoColumns(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoColumn_405(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoColumnMinMax_406(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoRowsCount_407(mut node: Ptr<YGNode>, mut count: usize) {
    ({
        facebook_yoga_StyleImpl::resizeGridAutoRows(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoRow_408(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut type_: YGGridTrackType,
    mut value: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_396(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoRowMinMax_409(
    mut node: Ptr<YGNode>,
    mut index: usize,
    mut minType: YGGridTrackType,
    mut minValue: f32,
    mut maxType: YGGridTrackType,
    mut maxValue: f32,
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_180((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_397(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_180((node).clone()) })) });
}
pub fn YGRoundValueToPixelGrid_410(
    mut value: f64,
    mut pointScaleFactor: f64,
    mut forceCeil: bool,
    mut forceFloor: bool,
) -> f32 {
    return ({ roundValueToPixelGrid_411(value, pointScaleFactor, forceCeil, forceFloor) });
}
thread_local!(
    pub static YGValueZero_2: Value<YGValue> = Rc::new(RefCell::new(YGValue {
        value: 0_f32,
        unit: YGUnit_YGUnitPoint,
    }));
);
thread_local!(
    pub static YGValueUndefined_1: Value<YGValue> = Rc::new(RefCell::new(YGValue {
        value: YGUndefined_23.with(|rc| *rc.borrow()),
        unit: YGUnit_YGUnitUndefined,
    }));
);
thread_local!(
    pub static YGValueAuto_0: Value<YGValue> = Rc::new(RefCell::new(YGValue {
        value: YGUndefined_23.with(|rc| *rc.borrow()),
        unit: YGUnit_YGUnitAuto,
    }));
);
pub fn YGFloatIsUndefined_412(mut value: f32) -> bool {
    return ({ isUndefined_105(value) });
}
pub fn resolveChildAlignment_413(
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
) -> facebook_yoga_Align {
    let mut align: facebook_yoga_Align = if (({
        facebook_yoga_StyleImpl::alignSelf(&({ facebook_yoga_NodeImpl::style_25(&child) }))
    }) == facebook_yoga_Align_Auto)
    {
        ({ facebook_yoga_StyleImpl::alignItems(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
    } else {
        ({ facebook_yoga_StyleImpl::alignSelf(&({ facebook_yoga_NodeImpl::style_25(&child) })) })
    };
    if ((({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
        == facebook_yoga_Display_Flex)
        && (align == facebook_yoga_Align_Baseline))
        && ({
            isColumn_129(
                ({
                    facebook_yoga_StyleImpl::flexDirection(
                        &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    )
                }),
            )
        })
    {
        return facebook_yoga_Align_FlexStart;
    }
    return align;
}
pub fn resolveChildJustification_414(
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
) -> facebook_yoga_Justify {
    return if (({
        facebook_yoga_StyleImpl::justifySelf(&({ facebook_yoga_NodeImpl::style_25(&child) }))
    }) == facebook_yoga_Justify_Auto)
    {
        ({ facebook_yoga_StyleImpl::justifyItems(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
    } else {
        ({ facebook_yoga_StyleImpl::justifySelf(&({ facebook_yoga_NodeImpl::style_25(&child) })) })
    };
}
pub fn fallbackAlignment_415(mut align: facebook_yoga_Align) -> facebook_yoga_Align {
    'switch: {
        match { align } {
            __v if __v == facebook_yoga_Align_SpaceBetween
                || __v == facebook_yoga_Align_Stretch =>
            {
                return facebook_yoga_Align_FlexStart;
            }
            __v if __v == facebook_yoga_Align_SpaceAround
                || __v == facebook_yoga_Align_SpaceEvenly =>
            {
                return facebook_yoga_Align_FlexStart;
            }
            _ => {
                return align;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn fallbackAlignment_416(mut align: facebook_yoga_Justify) -> facebook_yoga_Justify {
    'switch: {
        match { align } {
            __v if __v == facebook_yoga_Justify_SpaceBetween => {
                return facebook_yoga_Justify_FlexStart;
            }
            __v if __v == facebook_yoga_Justify_SpaceAround
                || __v == facebook_yoga_Justify_SpaceEvenly =>
            {
                return facebook_yoga_Justify_FlexStart;
            }
            _ => {
                return align;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn paddingAndBorderForAxis_417(
    mut node: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut widthSize: f32,
) -> f32 {
    return ({
        ({
            facebook_yoga_StyleImpl::computeInlineStartPaddingAndBorder(
                &({ facebook_yoga_NodeImpl::style_25(&node) }),
                axis,
                direction,
                widthSize,
            )
        })
    } + {
        ({
            facebook_yoga_StyleImpl::computeInlineEndPaddingAndBorder(
                &({ facebook_yoga_NodeImpl::style_25(&node) }),
                axis,
                direction,
                widthSize,
            )
        })
    });
}
pub fn boundAxisWithinMinAndMax_418(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut value: facebook_yoga_FloatOptional,
    mut axisSize: f32,
    mut widthSize: f32,
) -> facebook_yoga_FloatOptional {
    let mut min: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    let mut max: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    if ({ isColumn_129(axis) }) {
        min = ({
            facebook_yoga_StyleImpl::resolvedMinDimension(
                &({ facebook_yoga_NodeImpl::style_25(&node) }),
                direction,
                facebook_yoga_Dimension_Height,
                axisSize,
                widthSize,
            )
        });
        max = ({
            facebook_yoga_StyleImpl::resolvedMaxDimension(
                &({ facebook_yoga_NodeImpl::style_25(&node) }),
                direction,
                facebook_yoga_Dimension_Height,
                axisSize,
                widthSize,
            )
        });
    } else if ({ isRow_128(axis) }) {
        min = ({
            facebook_yoga_StyleImpl::resolvedMinDimension(
                &({ facebook_yoga_NodeImpl::style_25(&node) }),
                direction,
                facebook_yoga_Dimension_Width,
                axisSize,
                widthSize,
            )
        });
        max = ({
            facebook_yoga_StyleImpl::resolvedMaxDimension(
                &({ facebook_yoga_NodeImpl::style_25(&node) }),
                direction,
                facebook_yoga_Dimension_Width,
                axisSize,
                widthSize,
            )
        });
    }
    if ({
        let _lhs: facebook_yoga_FloatOptional = (max).clone();
        operator_ge_119(_lhs, facebook_yoga_FloatOptional::new_1({ 0_f32 }))
    }) && ({
        let _lhs: facebook_yoga_FloatOptional = (value).clone();
        operator_gt_118(_lhs, (max).clone())
    }) {
        return max;
    }
    if ({
        let _lhs: facebook_yoga_FloatOptional = (min).clone();
        operator_ge_119(_lhs, facebook_yoga_FloatOptional::new_1({ 0_f32 }))
    }) && ({
        let _lhs: facebook_yoga_FloatOptional = (value).clone();
        operator_lt_113(_lhs, (min).clone())
    }) {
        return min;
    }
    return value;
}
pub fn boundAxis_419(
    mut node: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut value: f32,
    mut axisSize: f32,
    mut widthSize: f32,
) -> f32 {
    return ({
        let _a: f32 = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        boundAxisWithinMinAndMax_418(
                            (node).clone(),
                            direction,
                            axis,
                            facebook_yoga_FloatOptional::new_1({ value }),
                            axisSize,
                            widthSize,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
        let _b: f32 = ({ paddingAndBorderForAxis_417((node).clone(), axis, direction, widthSize) });
        maxOrDefined_110(_a, _b)
    });
}
pub fn getPositionOfOppositeEdge_420(
    mut position: f32,
    mut axis: facebook_yoga_FlexDirection,
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut node: Ptr<facebook_yoga_Node>,
) -> f32 {
    return ({
        ({
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&containingNode) }),
                    ({ dimension_136(axis) }),
                )
            })
        } - {
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    ({ dimension_136(axis) }),
                )
            })
        })
    } - { position });
}
pub fn setChildTrailingPosition_421(
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
) {
    ({
        let _position: f32 = ({
            let _position: f32 = ({
                facebook_yoga_LayoutResultsImpl::position(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    ({ flexStartEdge_132(axis) }),
                )
            });
            let _axis: facebook_yoga_FlexDirection = axis;
            let _node: Ptr<facebook_yoga_Node> = (child).clone();
            getPositionOfOppositeEdge_420(_position, _axis, (node).clone(), _node)
        });
        let _edge: facebook_yoga_PhysicalEdge = ({ flexEndEdge_133(axis) });
        facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
    });
}
pub fn needsTrailingPosition_422(mut axis: facebook_yoga_FlexDirection) -> bool {
    return (axis == facebook_yoga_FlexDirection_RowReverse)
        || (axis == facebook_yoga_FlexDirection_ColumnReverse);
}
pub fn setFlexStartLayoutPosition_423(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: f32,
) {
    let mut position: f32 = ({
        ({
            facebook_yoga_StyleImpl::computeFlexStartMargin(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                axis,
                direction,
                containingBlockWidth,
            )
        })
    } + {
        ({
            facebook_yoga_LayoutResultsImpl::border(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexStartEdge_132(axis) }),
            )
        })
    });
    if (!({
        facebook_yoga_NodeImpl::hasErrata(
            &child,
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && (({
        facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&parent) }))
    }) != facebook_yoga_Display_Grid)
    {
        position += ({
            facebook_yoga_LayoutResultsImpl::padding(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexStartEdge_132(axis) }),
            )
        });
    }
    ({
        facebook_yoga_NodeImpl::setLayoutPosition(&child, position, ({ flexStartEdge_132(axis) }))
    });
}
pub fn setFlexEndLayoutPosition_424(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: f32,
) {
    let mut flexEndPosition: f32 = ({
        ({
            facebook_yoga_LayoutResultsImpl::border(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_133(axis) }),
            )
        })
    } + {
        ({
            facebook_yoga_StyleImpl::computeFlexEndMargin(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                axis,
                direction,
                containingBlockWidth,
            )
        })
    });
    if (!({
        facebook_yoga_NodeImpl::hasErrata(
            &child,
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && (({
        facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&parent) }))
    }) != facebook_yoga_Display_Grid)
    {
        flexEndPosition += ({
            facebook_yoga_LayoutResultsImpl::padding(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_133(axis) }),
            )
        });
    }
    ({
        let _position: f32 = ({
            getPositionOfOppositeEdge_420(flexEndPosition, axis, (parent).clone(), (child).clone())
        });
        let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_132(axis) });
        facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
    });
}
pub fn setCenterLayoutPosition_425(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: f32,
) {
    let mut parentContentBoxSize: f32 = ({
        ({
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                    ({ dimension_136(axis) }),
                )
            })
        } - {
            ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                    ({ flexStartEdge_132(axis) }),
                )
            })
        })
    } - {
        ({
            facebook_yoga_LayoutResultsImpl::border(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_133(axis) }),
            )
        })
    });
    if (!({
        facebook_yoga_NodeImpl::hasErrata(
            &child,
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && (({
        facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&parent) }))
    }) != facebook_yoga_Display_Grid)
    {
        parentContentBoxSize -= ({
            facebook_yoga_LayoutResultsImpl::padding(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexStartEdge_132(axis) }),
            )
        });
        parentContentBoxSize -= ({
            facebook_yoga_LayoutResultsImpl::padding(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_133(axis) }),
            )
        });
    }
    let mut childOuterSize: f32 = ({
        ({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                ({ dimension_136(axis) }),
            )
        })
    } + {
        ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                axis,
                containingBlockWidth,
            )
        })
    });
    let mut position: f32 = ({
        ({ ((parentContentBoxSize - childOuterSize) / 2_f32) } + {
            ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                    ({ flexStartEdge_132(axis) }),
                )
            })
        })
    } + {
        ({
            facebook_yoga_StyleImpl::computeFlexStartMargin(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                axis,
                direction,
                containingBlockWidth,
            )
        })
    });
    if (!({
        facebook_yoga_NodeImpl::hasErrata(
            &child,
            facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding,
        )
    })) && (({
        facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&parent) }))
    }) != facebook_yoga_Display_Grid)
    {
        position += ({
            facebook_yoga_LayoutResultsImpl::padding(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexStartEdge_132(axis) }),
            )
        });
    }
    ({
        facebook_yoga_NodeImpl::setLayoutPosition(&child, position, ({ flexStartEdge_132(axis) }))
    });
}
pub fn justifyAbsoluteChild_426(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: f32,
) {
    let mut justify: facebook_yoga_Justify = if (({
        facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&parent) }))
    }) == facebook_yoga_Display_Grid)
    {
        ({ resolveChildJustification_414((parent).clone(), (child).clone()) })
    } else {
        ({
            facebook_yoga_StyleImpl::justifyContent(
                &({ facebook_yoga_NodeImpl::style_25(&parent) }),
            )
        })
    };
    'switch: {
        match { justify } {
            __v if __v == facebook_yoga_Justify_Start
                || __v == facebook_yoga_Justify_Auto
                || __v == facebook_yoga_Justify_Stretch
                || __v == facebook_yoga_Justify_FlexStart
                || __v == facebook_yoga_Justify_SpaceBetween =>
            {
                ({
                    setFlexStartLayoutPosition_423(
                        (parent).clone(),
                        (child).clone(),
                        direction,
                        mainAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Justify_End || __v == facebook_yoga_Justify_FlexEnd => {
                ({
                    setFlexEndLayoutPosition_424(
                        (parent).clone(),
                        (child).clone(),
                        direction,
                        mainAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Justify_Center
                || __v == facebook_yoga_Justify_SpaceAround
                || __v == facebook_yoga_Justify_SpaceEvenly =>
            {
                ({
                    setCenterLayoutPosition_425(
                        (parent).clone(),
                        (child).clone(),
                        direction,
                        mainAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn alignAbsoluteChild_427(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: f32,
) {
    let mut itemAlign: facebook_yoga_Align =
        ({ resolveChildAlignment_413((parent).clone(), (child).clone()) });
    let mut parentWrap: facebook_yoga_Wrap =
        ({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_25(&parent) })) });
    if (parentWrap == facebook_yoga_Wrap_WrapReverse) {
        if (itemAlign == facebook_yoga_Align_FlexEnd) {
            itemAlign = facebook_yoga_Align_FlexStart;
        } else if (itemAlign != facebook_yoga_Align_Center) {
            itemAlign = facebook_yoga_Align_FlexEnd;
        }
    }
    'switch: {
        match { itemAlign } {
            __v if __v == facebook_yoga_Align_Start
                || __v == facebook_yoga_Align_Auto
                || __v == facebook_yoga_Align_FlexStart
                || __v == facebook_yoga_Align_Baseline
                || __v == facebook_yoga_Align_SpaceAround
                || __v == facebook_yoga_Align_SpaceBetween
                || __v == facebook_yoga_Align_Stretch
                || __v == facebook_yoga_Align_SpaceEvenly =>
            {
                ({
                    setFlexStartLayoutPosition_423(
                        (parent).clone(),
                        (child).clone(),
                        direction,
                        crossAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Align_End || __v == facebook_yoga_Align_FlexEnd => {
                ({
                    setFlexEndLayoutPosition_424(
                        (parent).clone(),
                        (child).clone(),
                        direction,
                        crossAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            __v if __v == facebook_yoga_Align_Center => {
                ({
                    setCenterLayoutPosition_425(
                        (parent).clone(),
                        (child).clone(),
                        direction,
                        crossAxis,
                        containingBlockWidth,
                    )
                });
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn positionAbsoluteChild_428(
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut isMainAxis: bool,
    mut containingBlockWidth: f32,
    mut containingBlockHeight: f32,
) {
    let mut isAxisRow: bool = ({ isRow_128(axis) });
    let mut containingBlockSize: f32 = if isAxisRow {
        containingBlockWidth
    } else {
        containingBlockHeight
    };
    if ({
        facebook_yoga_StyleImpl::isInlineStartPositionDefined(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            axis,
            direction,
        )
    }) && (!({
        facebook_yoga_StyleImpl::isInlineStartPositionAuto(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            axis,
            direction,
        )
    })) {
        let mut positionRelativeToInlineStart: f32 = ({
            ({
                ({
                    facebook_yoga_StyleImpl::computeInlineStartPosition(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        axis,
                        direction,
                        containingBlockSize,
                    )
                })
            } + {
                ({
                    facebook_yoga_StyleImpl::computeInlineStartBorder(
                        &({ facebook_yoga_NodeImpl::style_25(&containingNode) }),
                        axis,
                        direction,
                    )
                })
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeInlineStartMargin(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    axis,
                    direction,
                    containingBlockSize,
                )
            })
        });
        let mut positionRelativeToFlexStart: f32 =
            if (({ inlineStartEdge_134(axis, direction) }) != ({ flexStartEdge_132(axis) })) {
                ({
                    getPositionOfOppositeEdge_420(
                        positionRelativeToInlineStart,
                        axis,
                        (containingNode).clone(),
                        (child).clone(),
                    )
                })
            } else {
                positionRelativeToInlineStart
            };
        ({
            facebook_yoga_NodeImpl::setLayoutPosition(
                &child,
                positionRelativeToFlexStart,
                ({ flexStartEdge_132(axis) }),
            )
        });
    } else if ({
        facebook_yoga_StyleImpl::isInlineEndPositionDefined(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            axis,
            direction,
        )
    }) && (!({
        facebook_yoga_StyleImpl::isInlineEndPositionAuto(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            axis,
            direction,
        )
    })) {
        let mut positionRelativeToInlineStart: f32 = ({
            ({
                ({
                    ({
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_27(&containingNode) }),
                                ({ dimension_136(axis) }),
                            )
                        })
                    } - {
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                ({ dimension_136(axis) }),
                            )
                        })
                    })
                } - {
                    ({
                        facebook_yoga_StyleImpl::computeInlineEndBorder(
                            &({ facebook_yoga_NodeImpl::style_25(&containingNode) }),
                            axis,
                            direction,
                        )
                    })
                })
            } - {
                ({
                    facebook_yoga_StyleImpl::computeInlineEndMargin(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        axis,
                        direction,
                        containingBlockSize,
                    )
                })
            })
        } - {
            ({
                facebook_yoga_StyleImpl::computeInlineEndPosition(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    axis,
                    direction,
                    containingBlockSize,
                )
            })
        });
        let mut positionRelativeToFlexStart: f32 =
            if (({ inlineStartEdge_134(axis, direction) }) != ({ flexStartEdge_132(axis) })) {
                ({
                    getPositionOfOppositeEdge_420(
                        positionRelativeToInlineStart,
                        axis,
                        (containingNode).clone(),
                        (child).clone(),
                    )
                })
            } else {
                positionRelativeToInlineStart
            };
        ({
            facebook_yoga_NodeImpl::setLayoutPosition(
                &child,
                positionRelativeToFlexStart,
                ({ flexStartEdge_132(axis) }),
            )
        });
    } else {
        if isMainAxis {
            ({
                justifyAbsoluteChild_426(
                    (parent).clone(),
                    (child).clone(),
                    direction,
                    axis,
                    containingBlockWidth,
                )
            });
        } else {
            ({
                alignAbsoluteChild_427(
                    (parent).clone(),
                    (child).clone(),
                    direction,
                    axis,
                    containingBlockWidth,
                )
            });
        };
    }
}
pub fn layoutAbsoluteChild_429(
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut containingBlockWidth: f32,
    mut containingBlockHeight: f32,
    mut widthMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) {
    let mut mainAxis: facebook_yoga_FlexDirection =
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
            == facebook_yoga_Display_Grid)
        {
            ({ resolveDirection_130(facebook_yoga_FlexDirection_Row, direction) })
        } else {
            ({
                resolveDirection_130(
                    ({
                        facebook_yoga_StyleImpl::flexDirection(
                            &({ facebook_yoga_NodeImpl::style_25(&node) }),
                        )
                    }),
                    direction,
                )
            })
        };
    let mut crossAxis: facebook_yoga_FlexDirection =
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
            == facebook_yoga_Display_Grid)
        {
            facebook_yoga_FlexDirection_Column
        } else {
            ({ resolveCrossDirection_131(mainAxis, direction) })
        };
    let mut isMainAxisRow: bool = ({ isRow_128(mainAxis) });
    let mut childWidth: f32 = YGUndefined_23.with(|rc| *rc.borrow());
    let mut childHeight: f32 = YGUndefined_23.with(|rc| *rc.borrow());
    let mut childWidthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut childHeightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut marginRow: f32 = ({
        facebook_yoga_StyleImpl::computeMarginForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            facebook_yoga_FlexDirection_Row,
            containingBlockWidth,
        )
    });
    let mut marginColumn: f32 = ({
        facebook_yoga_StyleImpl::computeMarginForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            facebook_yoga_FlexDirection_Column,
            containingBlockWidth,
        )
    });
    if ({
        facebook_yoga_NodeImpl::hasDefiniteLength(
            &child,
            facebook_yoga_Dimension_Width,
            containingBlockWidth,
        )
    }) {
        childWidth = ({
            ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({
                            let _referenceLength: f32 = containingBlockWidth;
                            let _ownerWidth: f32 = containingBlockWidth;
                            facebook_yoga_NodeImpl::getResolvedDimension(
                                &child,
                                direction,
                                facebook_yoga_Dimension_Width,
                                _referenceLength,
                                _ownerWidth,
                            )
                        }),
                    ))
                    .as_pointer(),
                )
            })
        } + { marginRow });
    } else {
        if ((({
            facebook_yoga_StyleImpl::isFlexStartPositionDefined(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        }) && ({
            facebook_yoga_StyleImpl::isFlexEndPositionDefined(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        })) && (!({
            facebook_yoga_StyleImpl::isFlexStartPositionAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        }))) && (!({
            facebook_yoga_StyleImpl::isFlexEndPositionAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Row,
                direction,
            )
        })) {
            childWidth = ({
                ({
                    ({
                        facebook_yoga_LayoutResultsImpl::measuredDimension(
                            &({ facebook_yoga_NodeImpl::getLayout_27(&containingNode) }),
                            facebook_yoga_Dimension_Width,
                        )
                    })
                } - {
                    ({
                        ({
                            facebook_yoga_StyleImpl::computeFlexStartBorder(
                                &({ facebook_yoga_NodeImpl::style_25(&containingNode) }),
                                facebook_yoga_FlexDirection_Row,
                                direction,
                            )
                        })
                    } + {
                        ({
                            facebook_yoga_StyleImpl::computeFlexEndBorder(
                                &({ facebook_yoga_NodeImpl::style_25(&containingNode) }),
                                facebook_yoga_FlexDirection_Row,
                                direction,
                            )
                        })
                    })
                })
            } - {
                ({
                    ({
                        facebook_yoga_StyleImpl::computeFlexStartPosition(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            facebook_yoga_FlexDirection_Row,
                            direction,
                            containingBlockWidth,
                        )
                    })
                } + {
                    ({
                        facebook_yoga_StyleImpl::computeFlexEndPosition(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            facebook_yoga_FlexDirection_Row,
                            direction,
                            containingBlockWidth,
                        )
                    })
                })
            });
            let __rhs = ({
                let _axisSize: f32 = containingBlockWidth;
                let _widthSize: f32 = containingBlockWidth;
                boundAxis_419(
                    (child).clone(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    childWidth,
                    _axisSize,
                    _widthSize,
                )
            });
            childWidth = __rhs;
        }
    }
    if ({
        facebook_yoga_NodeImpl::hasDefiniteLength(
            &child,
            facebook_yoga_Dimension_Height,
            containingBlockHeight,
        )
    }) {
        childHeight = ({
            ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({
                            facebook_yoga_NodeImpl::getResolvedDimension(
                                &child,
                                direction,
                                facebook_yoga_Dimension_Height,
                                containingBlockHeight,
                                containingBlockWidth,
                            )
                        }),
                    ))
                    .as_pointer(),
                )
            })
        } + { marginColumn });
    } else {
        if ((({
            facebook_yoga_StyleImpl::isFlexStartPositionDefined(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        }) && ({
            facebook_yoga_StyleImpl::isFlexEndPositionDefined(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        })) && (!({
            facebook_yoga_StyleImpl::isFlexStartPositionAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        }))) && (!({
            facebook_yoga_StyleImpl::isFlexEndPositionAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Column,
                direction,
            )
        })) {
            childHeight = ({
                ({
                    ({
                        facebook_yoga_LayoutResultsImpl::measuredDimension(
                            &({ facebook_yoga_NodeImpl::getLayout_27(&containingNode) }),
                            facebook_yoga_Dimension_Height,
                        )
                    })
                } - {
                    ({
                        ({
                            facebook_yoga_StyleImpl::computeFlexStartBorder(
                                &({ facebook_yoga_NodeImpl::style_25(&containingNode) }),
                                facebook_yoga_FlexDirection_Column,
                                direction,
                            )
                        })
                    } + {
                        ({
                            facebook_yoga_StyleImpl::computeFlexEndBorder(
                                &({ facebook_yoga_NodeImpl::style_25(&containingNode) }),
                                facebook_yoga_FlexDirection_Column,
                                direction,
                            )
                        })
                    })
                })
            } - {
                ({
                    ({
                        facebook_yoga_StyleImpl::computeFlexStartPosition(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            facebook_yoga_FlexDirection_Column,
                            direction,
                            containingBlockHeight,
                        )
                    })
                } + {
                    ({
                        facebook_yoga_StyleImpl::computeFlexEndPosition(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            facebook_yoga_FlexDirection_Column,
                            direction,
                            containingBlockHeight,
                        )
                    })
                })
            });
            let __rhs = ({
                boundAxis_419(
                    (child).clone(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    childHeight,
                    containingBlockHeight,
                    containingBlockWidth,
                )
            });
            childHeight = __rhs;
        }
    }
    let childStyle: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&child) });
    if (((({ isUndefined_105(childWidth) }) as i32) ^ (({ isUndefined_105(childHeight) }) as i32))
        != 0)
    {
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                ))
                .as_pointer(),
            )
        }) {
            if ({ isUndefined_105(childWidth) }) {
                childWidth = ({ marginRow } + {
                    ({ (childHeight - marginColumn) } * {
                        ({
                            facebook_yoga_FloatOptionalImpl::unwrap(
                                &Rc::new(RefCell::new(
                                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                                ))
                                .as_pointer(),
                            )
                        })
                    })
                });
            } else if ({ isUndefined_105(childHeight) }) {
                childHeight = ({ marginColumn } + {
                    ({ (childWidth - marginRow) } / {
                        ({
                            facebook_yoga_FloatOptionalImpl::unwrap(
                                &Rc::new(RefCell::new(
                                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                                ))
                                .as_pointer(),
                            )
                        })
                    })
                });
            }
        }
    }
    if ({ isUndefined_105(childWidth) }) || ({ isUndefined_105(childHeight) }) {
        childWidthSizingMode = if ({ isUndefined_105(childWidth) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        childHeightSizingMode = if ({ isUndefined_105(childHeight) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        if ((((!(isMainAxisRow)) && ({ isUndefined_105(childWidth) }))
            && (widthMode != facebook_yoga_SizingMode_MaxContent))
            && ({ isDefined_107(containingBlockWidth) }))
            && (containingBlockWidth > 0_f32)
        {
            childWidth = containingBlockWidth;
            childWidthSizingMode = facebook_yoga_SizingMode_FitContent;
        }
        ({
            let _node: Ptr<facebook_yoga_Node> = (child).clone();
            let _availableWidth: f32 = childWidth;
            let _availableHeight: f32 = childHeight;
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: f32 = containingBlockWidth;
            let _ownerHeight: f32 = containingBlockHeight;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutInternal_430(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                false,
                facebook_yoga_LayoutPassReason_kAbsMeasureChild,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        childWidth = ({
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    facebook_yoga_Dimension_Width,
                )
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeMarginForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    facebook_yoga_FlexDirection_Row,
                    containingBlockWidth,
                )
            })
        });
        childHeight = ({
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    facebook_yoga_Dimension_Height,
                )
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeMarginForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    facebook_yoga_FlexDirection_Column,
                    containingBlockWidth,
                )
            })
        });
    }
    ({
        let _node: Ptr<facebook_yoga_Node> = (child).clone();
        let _availableWidth: f32 = childWidth;
        let _availableHeight: f32 = childHeight;
        let _ownerDirection: facebook_yoga_Direction = direction;
        let _ownerWidth: f32 = containingBlockWidth;
        let _ownerHeight: f32 = containingBlockHeight;
        let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
        let _depth: u32 = depth;
        let _generationCount: u32 = generationCount;
        calculateLayoutInternal_430(
            _node,
            _availableWidth,
            _availableHeight,
            _ownerDirection,
            facebook_yoga_SizingMode_StretchFit,
            facebook_yoga_SizingMode_StretchFit,
            _ownerWidth,
            _ownerHeight,
            true,
            facebook_yoga_LayoutPassReason_kAbsLayout,
            _layoutMarkerData,
            _depth,
            _generationCount,
        )
    });
    ({
        positionAbsoluteChild_428(
            (containingNode).clone(),
            (node).clone(),
            (child).clone(),
            direction,
            mainAxis,
            true,
            containingBlockWidth,
            containingBlockHeight,
        )
    });
    ({
        positionAbsoluteChild_428(
            (containingNode).clone(),
            (node).clone(),
            (child).clone(),
            direction,
            crossAxis,
            false,
            containingBlockWidth,
            containingBlockHeight,
        )
    });
}
pub fn layoutAbsoluteDescendants_431(
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut currentNode: Ptr<facebook_yoga_Node>,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut currentNodeDirection: facebook_yoga_Direction,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut currentDepth: u32,
    mut generationCount: u32,
    mut currentNodeLeftOffsetFromContainingBlock: f32,
    mut currentNodeTopOffsetFromContainingBlock: f32,
    mut containingNodeAvailableInnerWidth: f32,
    mut containingNodeAvailableInnerHeight: f32,
) -> bool {
    let mut hasNewLayout: bool = false;
    let __tmp_0: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
        RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&currentNode) })),
    );
    let __range2: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = __tmp_0.as_pointer();
    let __begin2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(&__range2) }),
        ));
    let __end2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(&__range2) }),
        ));
    'loop_: while ({
        let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            __begin2.as_pointer();
        operator_ne_78(_a, __end2.as_pointer())
    }) {
        let mut child: Ptr<facebook_yoga_Node> = ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                &__begin2.as_pointer(),
            )
        });
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
            == facebook_yoga_Display_None)
        {
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin2.as_pointer(),
                )
            });
            continue 'loop_;
        } else if (({
            facebook_yoga_StyleImpl::positionType(&({ facebook_yoga_NodeImpl::style_24(&child) }))
        }) == facebook_yoga_PositionType_Absolute)
        {
            let mut absoluteErrata: bool = ({
                facebook_yoga_NodeImpl::hasErrata(
                    &currentNode,
                    facebook_yoga_Errata_AbsolutePercentAgainstInnerSize,
                )
            });
            let mut containingBlockWidth: f32 = if absoluteErrata {
                containingNodeAvailableInnerWidth
            } else {
                ({
                    ({
                        facebook_yoga_LayoutResultsImpl::measuredDimension(
                            &({ facebook_yoga_NodeImpl::getLayout_26(&containingNode) }),
                            facebook_yoga_Dimension_Width,
                        )
                    })
                } - {
                    ({
                        facebook_yoga_StyleImpl::computeBorderForAxis(
                            &({ facebook_yoga_NodeImpl::style_24(&containingNode) }),
                            facebook_yoga_FlexDirection_Row,
                        )
                    })
                })
            };
            let mut containingBlockHeight: f32 = if absoluteErrata {
                containingNodeAvailableInnerHeight
            } else {
                ({
                    ({
                        facebook_yoga_LayoutResultsImpl::measuredDimension(
                            &({ facebook_yoga_NodeImpl::getLayout_26(&containingNode) }),
                            facebook_yoga_Dimension_Height,
                        )
                    })
                } - {
                    ({
                        facebook_yoga_StyleImpl::computeBorderForAxis(
                            &({ facebook_yoga_NodeImpl::style_24(&containingNode) }),
                            facebook_yoga_FlexDirection_Column,
                        )
                    })
                })
            };
            ({
                let _containingNode: Ptr<facebook_yoga_Node> = (containingNode).clone();
                let _node: Ptr<facebook_yoga_Node> = (currentNode).clone();
                let _child: Ptr<facebook_yoga_Node> = (child).clone();
                let _containingBlockWidth: f32 = containingBlockWidth;
                let _containingBlockHeight: f32 = containingBlockHeight;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _direction: facebook_yoga_Direction = currentNodeDirection;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _depth: u32 = currentDepth;
                let _generationCount: u32 = generationCount;
                layoutAbsoluteChild_429(
                    _containingNode,
                    _node,
                    _child,
                    _containingBlockWidth,
                    _containingBlockHeight,
                    _widthMode,
                    _direction,
                    _layoutMarkerData,
                    _depth,
                    _generationCount,
                )
            });
            let __rhs = (hasNewLayout) || ({ facebook_yoga_NodeImpl::getHasNewLayout(&child) });
            hasNewLayout = __rhs;
            let mut parentMainAxis: facebook_yoga_FlexDirection = ({
                resolveDirection_130(
                    ({
                        facebook_yoga_StyleImpl::flexDirection(
                            &({ facebook_yoga_NodeImpl::style_24(&currentNode) }),
                        )
                    }),
                    currentNodeDirection,
                )
            });
            let mut parentCrossAxis: facebook_yoga_FlexDirection =
                ({ resolveCrossDirection_131(parentMainAxis, currentNodeDirection) });
            if ({ needsTrailingPosition_422(parentMainAxis) }) {
                let mut mainInsetsDefined: bool = if ({ isRow_128(parentMainAxis) }) {
                    ({
                        facebook_yoga_StyleImpl::horizontalInsetsDefined(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        )
                    })
                } else {
                    ({
                        facebook_yoga_StyleImpl::verticalInsetsDefined(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        )
                    })
                };
                ({
                    setChildTrailingPosition_421(
                        if mainInsetsDefined {
                            (containingNode).clone()
                        } else {
                            (currentNode).clone()
                        },
                        (child).clone(),
                        parentMainAxis,
                    )
                });
            }
            if ({ needsTrailingPosition_422(parentCrossAxis) }) {
                let mut crossInsetsDefined: bool = if ({ isRow_128(parentCrossAxis) }) {
                    ({
                        facebook_yoga_StyleImpl::horizontalInsetsDefined(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        )
                    })
                } else {
                    ({
                        facebook_yoga_StyleImpl::verticalInsetsDefined(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        )
                    })
                };
                ({
                    setChildTrailingPosition_421(
                        if crossInsetsDefined {
                            (containingNode).clone()
                        } else {
                            (currentNode).clone()
                        },
                        (child).clone(),
                        parentCrossAxis,
                    )
                });
            }
            let mut childLeftPosition: f32 = ({
                facebook_yoga_LayoutResultsImpl::position(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
            let mut childTopPosition: f32 = ({
                facebook_yoga_LayoutResultsImpl::position(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    facebook_yoga_PhysicalEdge_Top,
                )
            });
            let mut childLeftOffsetFromParent: f32 = if ({
                facebook_yoga_StyleImpl::horizontalInsetsDefined(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                )
            }) {
                (childLeftPosition - currentNodeLeftOffsetFromContainingBlock)
            } else {
                childLeftPosition
            };
            let mut childTopOffsetFromParent: f32 = if ({
                facebook_yoga_StyleImpl::verticalInsetsDefined(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                )
            }) {
                (childTopPosition - currentNodeTopOffsetFromContainingBlock)
            } else {
                childTopPosition
            };
            ({
                facebook_yoga_NodeImpl::setLayoutPosition(
                    &child,
                    childLeftOffsetFromParent,
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
            ({
                facebook_yoga_NodeImpl::setLayoutPosition(
                    &child,
                    childTopOffsetFromParent,
                    facebook_yoga_PhysicalEdge_Top,
                )
            });
        } else if (({
            facebook_yoga_StyleImpl::positionType(&({ facebook_yoga_NodeImpl::style_24(&child) }))
        }) == facebook_yoga_PositionType_Static)
            && (!({ facebook_yoga_NodeImpl::alwaysFormsContainingBlock(&child) }))
        {
            ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&child) });
            let mut childDirection: facebook_yoga_Direction =
                ({ facebook_yoga_NodeImpl::resolveDirection(&child, currentNodeDirection) });
            let mut childLeftOffsetFromContainingBlock: f32 =
                ({ currentNodeLeftOffsetFromContainingBlock } + {
                    ({
                        facebook_yoga_LayoutResultsImpl::position(
                            &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                            facebook_yoga_PhysicalEdge_Left,
                        )
                    })
                });
            let mut childTopOffsetFromContainingBlock: f32 =
                ({ currentNodeTopOffsetFromContainingBlock } + {
                    ({
                        facebook_yoga_LayoutResultsImpl::position(
                            &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                            facebook_yoga_PhysicalEdge_Top,
                        )
                    })
                });
            let __rhs = ({
                let _containingNode: Ptr<facebook_yoga_Node> = (containingNode).clone();
                let _currentNode: Ptr<facebook_yoga_Node> = (child).clone();
                let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
                let _currentNodeDirection: facebook_yoga_Direction = childDirection;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _currentDepth: u32 = (currentDepth).wrapping_add(1_u32);
                let _generationCount: u32 = generationCount;
                let _currentNodeLeftOffsetFromContainingBlock: f32 =
                    childLeftOffsetFromContainingBlock;
                let _currentNodeTopOffsetFromContainingBlock: f32 =
                    childTopOffsetFromContainingBlock;
                let _containingNodeAvailableInnerWidth: f32 = containingNodeAvailableInnerWidth;
                let _containingNodeAvailableInnerHeight: f32 = containingNodeAvailableInnerHeight;
                layoutAbsoluteDescendants_431(
                    _containingNode,
                    _currentNode,
                    _widthSizingMode,
                    _currentNodeDirection,
                    _layoutMarkerData,
                    _currentDepth,
                    _generationCount,
                    _currentNodeLeftOffsetFromContainingBlock,
                    _currentNodeTopOffsetFromContainingBlock,
                    _containingNodeAvailableInnerWidth,
                    _containingNodeAvailableInnerHeight,
                )
            }) || (hasNewLayout);
            hasNewLayout = __rhs;
            ({ cleanupContentsNodesRecursively_432((child).clone(), hasNewLayout) });
            if hasNewLayout {
                ({ facebook_yoga_NodeImpl::setHasNewLayout(&child, hasNewLayout) });
            }
        }
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                &__begin2.as_pointer(),
            )
        });
    }
    return hasNewLayout;
}
impl facebook_yoga_Event {
    pub fn publish_3_7(
        mut node: Ptr<YGNode>,
        eventData: Option<
            Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_>,
        >,
    ) {
        let mut __eventData_default: Option<
            Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_>,
        > = None;
        let eventData : Ptr< facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_  >  = eventData.unwrap_or_else(|| __eventData_default.insert(Rc::new(RefCell::new(<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_  >::default() ))).as_pointer()) ;
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_3({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((7_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
    pub fn publish_3_8(
        mut node: Ptr<YGNode>,
        eventData: Option<Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_>>,
    ) {
        let mut __eventData_default: Option<
            Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_>,
        > = None;
        let eventData : Ptr< facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_  >  = eventData.unwrap_or_else(|| __eventData_default.insert(Rc::new(RefCell::new(<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_  >::default() ))).as_pointer()) ;
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_4({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((8_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_ {}
impl facebook_yoga_Event_Data {
    pub fn new_3(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
impl facebook_yoga_Event_Data {
    pub fn new_4(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
pub fn calculateBaseline_433(mut node: Ptr<facebook_yoga_Node>) -> f32 {
    if ({ facebook_yoga_NodeImpl::hasBaselineFunc(&node) }) {
        ({ facebook_yoga_Event::publish_3_7(node.reinterpret_cast::<YGNode>(), None) });
        let mut baseline: f32 = ({
            let _width: f32 = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_Dimension_Width,
                )
            });
            let _height: f32 = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_Dimension_Height,
                )
            });
            facebook_yoga_NodeImpl::baseline(&node, _width, _height)
        });
        ({ facebook_yoga_Event::publish_3_8(node.reinterpret_cast::<YGNode>(), None) });
        ({
            assertFatalWithNode_213(
                (node).clone(),
                !(baseline.is_nan()),
                Ptr::<i8>::from_string_literal(
                    b"Expect custom baseline function to not return NaN",
                ),
            )
        });
        return baseline;
    }
    let mut baselineChild: Ptr<facebook_yoga_Node> = Ptr::<facebook_yoga_Node>::null();
    let __tmp_0: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
        RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
    );
    let __range2: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = __tmp_0.as_pointer();
    let __begin2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(&__range2) }),
        ));
    let __end2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(&__range2) }),
        ));
    'loop_: while ({
        let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            __begin2.as_pointer();
        operator_ne_78(_a, __end2.as_pointer())
    }) {
        let mut child: Ptr<facebook_yoga_Node> = ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                &__begin2.as_pointer(),
            )
        });
        if (({ facebook_yoga_NodeImpl::getLineIndex(&child) }) > 0_usize) {
            break;
        }
        if (({
            facebook_yoga_StyleImpl::positionType(&({ facebook_yoga_NodeImpl::style_24(&child) }))
        }) == facebook_yoga_PositionType_Absolute)
        {
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin2.as_pointer(),
                )
            });
            continue 'loop_;
        }
        if (({ resolveChildAlignment_413((node).clone(), (child).clone()) })
            == facebook_yoga_Align_Baseline)
            || ({ facebook_yoga_NodeImpl::isReferenceBaseline(&child) })
        {
            baselineChild = (child).clone();
            break;
        }
        if (baselineChild).is_null() {
            baselineChild = (child).clone();
        }
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                &__begin2.as_pointer(),
            )
        });
    }
    if (baselineChild).is_null() {
        return ({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                facebook_yoga_Dimension_Height,
            )
        });
    }
    let mut baseline: f32 = ({ calculateBaseline_433((baselineChild).clone()) });
    return ({ baseline } + {
        ({
            facebook_yoga_LayoutResultsImpl::position(
                &({ facebook_yoga_NodeImpl::getLayout_26(&baselineChild) }),
                facebook_yoga_PhysicalEdge_Top,
            )
        })
    });
}
pub fn isBaselineLayout_434(mut node: Ptr<facebook_yoga_Node>) -> bool {
    if ({
        isColumn_129(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                )
            }),
        )
    }) {
        return false;
    }
    if (({ facebook_yoga_StyleImpl::alignItems(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
        == facebook_yoga_Align_Baseline)
    {
        return true;
    }
    let __tmp_1: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
        RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
    );
    let __range2: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = __tmp_1.as_pointer();
    let __begin2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(&__range2) }),
        ));
    let __end2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(&__range2) }),
        ));
    'loop_: while ({
        let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            __begin2.as_pointer();
        operator_ne_78(_a, __end2.as_pointer())
    }) {
        let mut child: Ptr<facebook_yoga_Node> = ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                &__begin2.as_pointer(),
            )
        });
        if (({
            facebook_yoga_StyleImpl::positionType(&({ facebook_yoga_NodeImpl::style_24(&child) }))
        }) != facebook_yoga_PositionType_Absolute)
            && (({
                facebook_yoga_StyleImpl::alignSelf(&({ facebook_yoga_NodeImpl::style_24(&child) }))
            }) == facebook_yoga_Align_Baseline)
        {
            return true;
        }
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                &__begin2.as_pointer(),
            )
        });
    }
    return false;
}
pub fn sizeIsExactAndMatchesOldMeasuredSize_435(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: f32,
    mut lastComputedSize: f32,
) -> bool {
    return (sizeMode == facebook_yoga_SizingMode_StretchFit)
        && ({ inexactEquals_111(size, lastComputedSize) });
}
pub fn oldSizeIsMaxContentAndStillFits_436(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: f32,
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastComputedSize: f32,
) -> bool {
    return ((sizeMode == facebook_yoga_SizingMode_FitContent)
        && (lastSizeMode == facebook_yoga_SizingMode_MaxContent))
        && ((size >= lastComputedSize) || ({ inexactEquals_111(size, lastComputedSize) }));
}
pub fn newSizeIsStricterAndStillValid_437(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: f32,
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastSize: f32,
    mut lastComputedSize: f32,
) -> bool {
    return ((((((lastSizeMode == facebook_yoga_SizingMode_FitContent)
        && (sizeMode == facebook_yoga_SizingMode_FitContent))
        && ({ isDefined_107(lastSize) }))
        && ({ isDefined_107(size) }))
        && ({ isDefined_107(lastComputedSize) }))
        && (lastSize > size))
        && ((lastComputedSize <= size) || ({ inexactEquals_111(size, lastComputedSize) }));
}
pub fn canUseCachedMeasurement_242(
    mut widthMode: facebook_yoga_SizingMode,
    mut availableWidth: f32,
    mut heightMode: facebook_yoga_SizingMode,
    mut availableHeight: f32,
    mut lastWidthMode: facebook_yoga_SizingMode,
    mut lastAvailableWidth: f32,
    mut lastHeightMode: facebook_yoga_SizingMode,
    mut lastAvailableHeight: f32,
    mut lastComputedWidth: f32,
    mut lastComputedHeight: f32,
    mut marginRow: f32,
    mut marginColumn: f32,
    mut config: Ptr<facebook_yoga_Config>,
) -> bool {
    if (({ isDefined_107(lastComputedHeight) }) && (lastComputedHeight < 0_f32))
        || (({ isDefined_107(lastComputedWidth) }) && (lastComputedWidth < 0_f32))
    {
        return false;
    }
    let mut pointScaleFactor: f32 = ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&config) });
    let mut useRoundedComparison: bool = (!((config).is_null())) && (pointScaleFactor != 0_f32);
    let mut effectiveWidth: f32 = if useRoundedComparison {
        ({
            roundValueToPixelGrid_411(
                (availableWidth as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        availableWidth
    };
    let mut effectiveHeight: f32 = if useRoundedComparison {
        ({
            roundValueToPixelGrid_411(
                (availableHeight as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        availableHeight
    };
    let mut effectiveLastWidth: f32 = if useRoundedComparison {
        ({
            roundValueToPixelGrid_411(
                (lastAvailableWidth as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        lastAvailableWidth
    };
    let mut effectiveLastHeight: f32 = if useRoundedComparison {
        ({
            roundValueToPixelGrid_411(
                (lastAvailableHeight as f64),
                (pointScaleFactor as f64),
                false,
                false,
            )
        })
    } else {
        lastAvailableHeight
    };
    let mut hasSameWidthSpec: bool =
        (lastWidthMode == widthMode) && ({ inexactEquals_111(effectiveLastWidth, effectiveWidth) });
    let mut hasSameHeightSpec: bool = (lastHeightMode == heightMode)
        && ({ inexactEquals_111(effectiveLastHeight, effectiveHeight) });
    let mut widthIsCompatible: bool = (((hasSameWidthSpec)
        || ({
            sizeIsExactAndMatchesOldMeasuredSize_435(
                widthMode,
                (availableWidth - marginRow),
                lastComputedWidth,
            )
        }))
        || ({
            oldSizeIsMaxContentAndStillFits_436(
                widthMode,
                (availableWidth - marginRow),
                lastWidthMode,
                lastComputedWidth,
            )
        }))
        || ({
            newSizeIsStricterAndStillValid_437(
                widthMode,
                (availableWidth - marginRow),
                lastWidthMode,
                lastAvailableWidth,
                lastComputedWidth,
            )
        });
    let mut heightIsCompatible: bool = (((hasSameHeightSpec)
        || ({
            sizeIsExactAndMatchesOldMeasuredSize_435(
                heightMode,
                (availableHeight - marginColumn),
                lastComputedHeight,
            )
        }))
        || ({
            oldSizeIsMaxContentAndStillFits_436(
                heightMode,
                (availableHeight - marginColumn),
                lastHeightMode,
                lastComputedHeight,
            )
        }))
        || ({
            newSizeIsStricterAndStillValid_437(
                heightMode,
                (availableHeight - marginColumn),
                lastHeightMode,
                lastAvailableHeight,
                lastComputedHeight,
            )
        });
    return (widthIsCompatible) && (heightIsCompatible);
}
impl facebook_yoga_Event {
    pub fn publish_3_5(
        mut node: Ptr<YGNode>,
        eventData: Option<
            Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_>,
        >,
    ) {
        let mut __eventData_default: Option<
            Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_>,
        > = None;
        let eventData : Ptr< facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_  >  = eventData.unwrap_or_else(|| __eventData_default.insert(Rc::new(RefCell::new(<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_  >::default() ))).as_pointer()) ;
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_5({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((5_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
    pub fn publish_3_6(
        mut node: Ptr<YGNode>,
        eventData: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_>,
    ) {
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_6({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((6_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
    pub fn publish_3_2(
        mut node: Ptr<YGNode>,
        eventData: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_>,
    ) {
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_7({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((2_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
    pub fn publish_3_3(
        mut node: Ptr<YGNode>,
        eventData: Option<Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_>>,
    ) {
        let mut __eventData_default: Option<
            Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_>,
        > = None;
        let eventData : Ptr< facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_  >  = eventData.unwrap_or_else(|| __eventData_default.insert(Rc::new(RefCell::new(<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_  >::default() ))).as_pointer()) ;
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_8({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((3_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
    pub fn publish_3_4(
        mut node: Ptr<YGNode>,
        eventData: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_>,
    ) {
        ({
            let _arg2: Value<facebook_yoga_Event_Data> =
                Rc::new(RefCell::new(facebook_yoga_Event_Data::new_9({
                    (eventData).clone()
                })));
            facebook_yoga_Event::publish(
                (node).clone(),
                ((4_u32) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_ {}
impl facebook_yoga_Event_Data {
    pub fn new_5(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
impl facebook_yoga_Event_Data {
    pub fn new_6(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
impl facebook_yoga_Event_Data {
    pub fn new_7(data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_>) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
impl facebook_yoga_Event_Data {
    pub fn new_8(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
impl facebook_yoga_Event_Data {
    pub fn new_9(
        data: Ptr<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_>,
    ) -> Self {
        Self {
            data_: (data).to_any(),
        }
    }
}
pub fn minOrDefined_438(a: f32, b: f32) -> f32 {
    let a: Value<f32> = Rc::new(RefCell::new(a));
    let b: Value<f32> = Rc::new(RefCell::new(b));
    if ({ isDefined_107((*a.borrow())) }) && ({ isDefined_107((*b.borrow())) }) {
        return ({
            let __a = a.as_pointer();
            let __b = b.as_pointer();
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }
        .read());
    }
    return if ({ isUndefined_105((*a.borrow())) }) {
        (*b.borrow())
    } else {
        (*a.borrow())
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct facebook_yoga_FlexLineRunningLayout {
    #[offset(0)]
    pub totalFlexGrowFactors: f32,
    #[offset(4)]
    pub totalFlexShrinkScaledFactors: f32,
    #[offset(8)]
    pub remainingFreeSpace: f32,
    #[offset(12)]
    pub mainDim: f32,
    #[offset(16)]
    pub crossDim: f32,
}
impl Default for facebook_yoga_FlexLineRunningLayout {
    fn default() -> Self {
        {
            facebook_yoga_FlexLineRunningLayout {
                totalFlexGrowFactors: 0_f32,
                totalFlexShrinkScaledFactors: 0_f32,
                remainingFreeSpace: 0_f32,
                mainDim: 0_f32,
                crossDim: 0_f32,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(64)]
pub struct facebook_yoga_FlexLine {
    #[offset(0)]
    #[byte_size(24)]
    pub itemsInFlow: Vec<Ptr<facebook_yoga_Node>>,
    #[offset(24)]
    pub sizeConsumed: f32,
    #[offset(32)]
    pub numberOfAutoMargins: usize,
    #[offset(40)]
    #[byte_size(20)]
    pub layout: facebook_yoga_FlexLineRunningLayout,
}
impl facebook_yoga_FlexLine {
    pub fn move_from(_a0: Ptr<facebook_yoga_FlexLine>) -> Self {
        Self {
            itemsInFlow: (*_a0.upgrade().deref()).itemsInFlow.clone(),
            sizeConsumed: { (*_a0.upgrade().deref()).sizeConsumed },
            numberOfAutoMargins: { (*_a0.upgrade().deref()).numberOfAutoMargins },
            layout: { (*_a0.upgrade().deref()).layout.clone() },
        }
    }
}
impl Default for facebook_yoga_FlexLine {
    fn default() -> Self {
        {
            facebook_yoga_FlexLine {
                itemsInFlow: Vec::new(),
                sizeConsumed: 0_f32,
                numberOfAutoMargins: 0_usize,
                layout: <facebook_yoga_FlexLineRunningLayout>::default(),
            }
        }
    }
}
thread_local!(
    pub static gCurrentGenerationCount_439: Value<u32> = Rc::new(RefCell::new(0_u32));
);
pub fn hasAutoHorizontalMargin_440(style: Ptr<facebook_yoga_Style>) -> bool {
    let __tmp_0: Value<Vec<facebook_yoga_Direction>> = Rc::new(RefCell::new(Vec::from([
        facebook_yoga_Direction_LTR,
        facebook_yoga_Direction_RTL,
    ])));
    let __range2: Ptr<Vec<facebook_yoga_Direction>> = __tmp_0.as_pointer();
    'loop_: for direction in IterRange::new(__range2.decay(), __range2.decay().to_end()) {
        let mut direction: facebook_yoga_Direction = (direction.read());
        if ({
            let _direction: facebook_yoga_Direction = direction;
            facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                &style,
                facebook_yoga_FlexDirection_Row,
                _direction,
            )
        }) || ({
            let _direction: facebook_yoga_Direction = direction;
            facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                &style,
                facebook_yoga_FlexDirection_Row,
                _direction,
            )
        }) {
            return true;
        }
    }
    return false;
}
pub fn isColumnStretchEdge_441(
    mut owner: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
) -> bool {
    if ((owner).is_null()) || ((child).is_null()) {
        return false;
    }
    let ownerStyle: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_25(&owner) });
    let childStyle: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_25(&child) });
    let childWidth: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
        ({ facebook_yoga_NodeImpl::getProcessedDimension(&child, facebook_yoga_Dimension_Width) }),
    ));
    return (((((((({ facebook_yoga_StyleImpl::display(&ownerStyle) })
        == facebook_yoga_Display_Flex)
        && ({ isColumn_129(({ facebook_yoga_StyleImpl::flexDirection(&ownerStyle) })) }))
        && (({ facebook_yoga_StyleImpl::flexWrap(&ownerStyle) })
            == facebook_yoga_Wrap_NoWrap))
        && (({ facebook_yoga_StyleImpl::positionType(&childStyle) })
            != facebook_yoga_PositionType_Absolute))
        && (!({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                ))
                .as_pointer(),
            )
        })))
        && (({ facebook_yoga_StyleSizeLengthImpl::isAuto(&childWidth.as_pointer()) })
            || ({ facebook_yoga_StyleSizeLengthImpl::isUndefined(&childWidth.as_pointer()) })))
        && (!({ hasAutoHorizontalMargin_440((childStyle).clone()) })))
        && (({ resolveChildAlignment_413(owner, child) }) == facebook_yoga_Align_Stretch);
}
pub fn isInColumnStretchScrollSubtree_442(mut node: Ptr<facebook_yoga_Node>) -> bool {
    let mut current: Ptr<facebook_yoga_Node> = (node).clone();
    'loop_: while !((current).is_null()) {
        let mut owner: Ptr<facebook_yoga_Node> = ({ facebook_yoga_NodeImpl::getOwner(&current) });
        'loop_: while (!((owner).is_null()))
            && (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&owner) }))
            }) == facebook_yoga_Display_Contents)
        {
            let __rhs = ({ facebook_yoga_NodeImpl::getOwner(&owner) });
            owner = __rhs;
        }
        if ((owner).is_null())
            || (!({ isColumnStretchEdge_441((owner).clone(), (current).clone()) }))
        {
            return false;
        }
        if (({
            facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_24(&owner) }))
        }) == facebook_yoga_Overflow_Scroll)
        {
            return true;
        }
        current = (owner).clone();
    }
    return false;
}
pub fn isNonZeroLength_443(length: Ptr<facebook_yoga_StyleLength>) -> bool {
    return ({ facebook_yoga_StyleLengthImpl::isAuto(&length) })
        || (({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleLengthImpl::value(&length) }),
                ))
                .as_pointer(),
            )
        }) && (({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleLengthImpl::value(&length) }),
                ))
                .as_pointer(),
            )
        }) != 0_f32));
}
pub fn hasNonZeroVerticalSpacing_444(style: Ptr<facebook_yoga_Style>) -> bool {
    let verticalEdges: Value<Vec<facebook_yoga_Edge>> = Rc::new(RefCell::new(Vec::from([
        facebook_yoga_Edge_Top,
        facebook_yoga_Edge_Bottom,
        facebook_yoga_Edge_Vertical,
        facebook_yoga_Edge_All,
    ])));
    'loop_: for edge in IterRange::new(verticalEdges.decay(), verticalEdges.decay().to_end()) {
        let mut edge: facebook_yoga_Edge = (edge.read());
        if (({
            let _length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_StyleImpl::margin(&style, _edge)
                }),
            ));
            isNonZeroLength_443(_length.as_pointer())
        }) || ({
            let _length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_StyleImpl::padding(&style, _edge)
                }),
            ));
            isNonZeroLength_443(_length.as_pointer())
        })) || ({
            let _length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_StyleImpl::border(&style, _edge)
                }),
            ));
            isNonZeroLength_443(_length.as_pointer())
        }) {
            return true;
        }
    }
    return false;
}
pub fn hasPercentageLength_445(style: Ptr<facebook_yoga_Style>) -> bool {
    let edges: Value<Vec<facebook_yoga_Edge>> = Rc::new(RefCell::new(Vec::from([
        facebook_yoga_Edge_Left,
        facebook_yoga_Edge_Top,
        facebook_yoga_Edge_Right,
        facebook_yoga_Edge_Bottom,
        facebook_yoga_Edge_Start,
        facebook_yoga_Edge_End,
        facebook_yoga_Edge_Horizontal,
        facebook_yoga_Edge_Vertical,
        facebook_yoga_Edge_All,
    ])));
    'loop_: for edge in IterRange::new(edges.decay(), edges.decay().to_end()) {
        let mut edge: facebook_yoga_Edge = (edge.read());
        if ((({
            facebook_yoga_StyleLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_Edge = edge;
                        facebook_yoga_StyleImpl::margin(&style, _edge)
                    }),
                ))
                .as_pointer(),
            )
        }) || ({
            facebook_yoga_StyleLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_Edge = edge;
                        facebook_yoga_StyleImpl::position(&style, _edge)
                    }),
                ))
                .as_pointer(),
            )
        })) || ({
            facebook_yoga_StyleLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_Edge = edge;
                        facebook_yoga_StyleImpl::padding(&style, _edge)
                    }),
                ))
                .as_pointer(),
            )
        })) || ({
            facebook_yoga_StyleLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_Edge = edge;
                        facebook_yoga_StyleImpl::border(&style, _edge)
                    }),
                ))
                .as_pointer(),
            )
        }) {
            return true;
        }
    }
    let dimensions: Value<Vec<facebook_yoga_Dimension>> = Rc::new(RefCell::new(Vec::from([
        facebook_yoga_Dimension_Width,
        facebook_yoga_Dimension_Height,
    ])));
    'loop_: for dimension in IterRange::new(dimensions.decay(), dimensions.decay().to_end()) {
        let mut dimension: facebook_yoga_Dimension = (dimension.read());
        if (({
            facebook_yoga_StyleSizeLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _axis: facebook_yoga_Dimension = dimension;
                        facebook_yoga_StyleImpl::dimension(&style, _axis)
                    }),
                ))
                .as_pointer(),
            )
        }) || ({
            facebook_yoga_StyleSizeLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _axis: facebook_yoga_Dimension = dimension;
                        facebook_yoga_StyleImpl::minDimension(&style, _axis)
                    }),
                ))
                .as_pointer(),
            )
        })) || ({
            facebook_yoga_StyleSizeLengthImpl::isPercent(
                &Rc::new(RefCell::new(
                    ({
                        let _axis: facebook_yoga_Dimension = dimension;
                        facebook_yoga_StyleImpl::maxDimension(&style, _axis)
                    }),
                ))
                .as_pointer(),
            )
        }) {
            return true;
        }
    }
    return ((({
        facebook_yoga_StyleSizeLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::flexBasis(&style) }),
            ))
            .as_pointer(),
        )
    }) || ({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::gap(&style, facebook_yoga_Gutter_Column) }),
            ))
            .as_pointer(),
        )
    })) || ({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::gap(&style, facebook_yoga_Gutter_Row) }),
            ))
            .as_pointer(),
        )
    })) || ({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::gap(&style, facebook_yoga_Gutter_All) }),
            ))
            .as_pointer(),
        )
    });
}
pub fn hasNonZeroFlex_446(node: Ptr<facebook_yoga_Node>) -> bool {
    let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_25(&node) });
    let flex: Value<facebook_yoga_FloatOptional> =
        Rc::new(RefCell::new(({ facebook_yoga_StyleImpl::flex(&style) })));
    let flexGrow: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::flexGrow(&style) }),
    ));
    let flexShrink: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::flexShrink(&style) }),
    ));
    let mut config: Ptr<facebook_yoga_Config> = ({ facebook_yoga_NodeImpl::getConfig(&node) });
    let mut canGrow: bool =
        if ({ facebook_yoga_FloatOptionalImpl::isDefined(&flexGrow.as_pointer()) }) {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&flexGrow.as_pointer()) }) != 0_f32)
        } else {
            ({ facebook_yoga_FloatOptionalImpl::isDefined(&flex.as_pointer()) })
                && (({ facebook_yoga_FloatOptionalImpl::unwrap(&flex.as_pointer()) }) > 0_f32)
        };
    let mut canShrink: bool =
        if ({ facebook_yoga_FloatOptionalImpl::isDefined(&flexShrink.as_pointer()) }) {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&flexShrink.as_pointer()) }) != 0_f32)
        } else {
            ((!((config).is_null())) && ({ facebook_yoga_ConfigImpl::useWebDefaults(&config) }))
                || (({ facebook_yoga_FloatOptionalImpl::isDefined(&flex.as_pointer()) })
                    && (({ facebook_yoga_FloatOptionalImpl::unwrap(&flex.as_pointer()) }) < 0_f32))
        };
    return (canGrow) || (canShrink);
}
pub fn isHeightFitContentIndependent_447(node: Ptr<facebook_yoga_Node>) -> bool {
    let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_25(&node) });
    let height: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::dimension(&style, facebook_yoga_Dimension_Height) }),
    ));
    let flexBasis: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::flexBasis(&style) }),
    ));
    let mut hasRelativePercentPosition: bool = ((({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::position(&style, facebook_yoga_Edge_Top) }),
            ))
            .as_pointer(),
        )
    }) || ({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::position(&style, facebook_yoga_Edge_Bottom) }),
            ))
            .as_pointer(),
        )
    })) || ({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::position(&style, facebook_yoga_Edge_Vertical) }),
            ))
            .as_pointer(),
        )
    })) || ({
        facebook_yoga_StyleLengthImpl::isPercent(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::position(&style, facebook_yoga_Edge_All) }),
            ))
            .as_pointer(),
        )
    });
    return (((((((((((((((((((((((!({
        facebook_yoga_NodeImpl::hasMeasureFunc(&node)
    })) && (!({
        facebook_yoga_NodeImpl::hasMinContentMeasureFunc(&node)
    }))) && (!({
        facebook_yoga_NodeImpl::hasBaselineFunc(&node)
    }))) && (!({
        facebook_yoga_NodeImpl::isReferenceBaseline(&node)
    }))) && (({
        facebook_yoga_StyleSizeLengthImpl::isAuto(&height.as_pointer())
    }) || ({
        facebook_yoga_StyleSizeLengthImpl::isUndefined(&height.as_pointer())
    }))) && ({
        facebook_yoga_StyleSizeLengthImpl::isUndefined(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::minDimension(&style, facebook_yoga_Dimension_Height) }),
            ))
            .as_pointer(),
        )
    })) && ({
        facebook_yoga_StyleSizeLengthImpl::isUndefined(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::maxDimension(&style, facebook_yoga_Dimension_Height) }),
            ))
            .as_pointer(),
        )
    })) && (({
        facebook_yoga_StyleSizeLengthImpl::isAuto(&flexBasis.as_pointer())
    }) || ({
        facebook_yoga_StyleSizeLengthImpl::isUndefined(&flexBasis.as_pointer())
    }))) && (!({ hasNonZeroFlex_446((node).clone()) })))
        && (({ facebook_yoga_StyleImpl::boxSizing(&style) })
            == facebook_yoga_BoxSizing_BorderBox))
        && (!({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::aspectRatio(&style) }),
                ))
                .as_pointer(),
            )
        })))
        && (({ facebook_yoga_StyleImpl::positionType(&style) })
            != facebook_yoga_PositionType_Absolute))
        && (({ facebook_yoga_StyleImpl::overflow(&style) })
            != facebook_yoga_Overflow_Scroll))
        && (({ facebook_yoga_StyleImpl::display(&style) })
            == facebook_yoga_Display_Flex))
        && ({ isColumn_129(({ facebook_yoga_StyleImpl::flexDirection(&style) })) }))
        && (({ facebook_yoga_StyleImpl::alignItems(&style) })
            == facebook_yoga_Align_Stretch))
        && ((({ facebook_yoga_StyleImpl::alignSelf(&style) }) == facebook_yoga_Align_Auto)
            || (({ facebook_yoga_StyleImpl::alignSelf(&style) })
                == facebook_yoga_Align_Stretch)))
        && (({ facebook_yoga_StyleImpl::justifyContent(&style) })
            == facebook_yoga_Justify_FlexStart))
        && (({ facebook_yoga_StyleImpl::flexWrap(&style) }) == facebook_yoga_Wrap_NoWrap))
        && (!({
            facebook_yoga_StyleLengthImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::gap(&style, facebook_yoga_Gutter_All) }),
                ))
                .as_pointer(),
            )
        })))
        && (!({
            facebook_yoga_StyleLengthImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::gap(&style, facebook_yoga_Gutter_Row) }),
                ))
                .as_pointer(),
            )
        })))
        && (!(hasRelativePercentPosition)))
        && (!({ hasNonZeroVerticalSpacing_444((style).clone()) })))
        && (!({ hasPercentageLength_445((style).clone()) }));
}
pub fn canSkipHeightFitContent_448(mut root: Ptr<facebook_yoga_Node>) -> bool {
    if (root).is_null() {
        return false;
    }
    let mut maxPendingNodes: usize = 64_usize;
    let stack: Value<Vec<Ptr<facebook_yoga_Node>>> = Rc::new(RefCell::new(Vec::from([
        (root).clone(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
        Ptr::<facebook_yoga_Node>::null(),
    ])));
    let mut stackSize: usize = 1_usize;
    'loop_: while (stackSize > 0_usize) {
        let mut node: Ptr<facebook_yoga_Node> =
            (stack.decay().offset(stackSize.prefix_dec() as isize).read());
        if ((node).is_null()) || (!({ isHeightFitContentIndependent_447((node).clone()) })) {
            return false;
        }
        let __tmp_1: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
            RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
        );
        let __range3: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> =
            __tmp_1.as_pointer();
        let __begin3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(&__range3) }),
            ));
        let __end3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(&__range3) }),
            ));
        'loop_: while ({
            let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                __begin3.as_pointer();
            operator_ne_78(_a, __end3.as_pointer())
        }) {
            let mut child: Ptr<facebook_yoga_Node> = ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                    &__begin3.as_pointer(),
                )
            });
            if (stackSize == (*stack.borrow()).len()) {
                return false;
            }
            stack
                .decay()
                .offset(stackSize.postfix_inc() as isize)
                .write((child).clone());
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin3.as_pointer(),
                )
            });
        }
    }
    return true;
}
pub fn constrainMaxSizeForMode_449(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut ownerAxisSize: f32,
    mut ownerWidth: f32,
    mut mode: Ptr<facebook_yoga_SizingMode>,
    mut size: Ptr<f32>,
) {
    let maxSize: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            let _lhs: facebook_yoga_FloatOptional = ({
                facebook_yoga_StyleImpl::resolvedMaxDimension(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    direction,
                    ({ dimension_136(axis) }),
                    ownerAxisSize,
                    ownerWidth,
                )
            });
            let _rhs: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
                ({
                    facebook_yoga_StyleImpl::computeMarginForAxis(
                        &({ facebook_yoga_NodeImpl::style_25(&node) }),
                        axis,
                        ownerWidth,
                    )
                })
            });
            operator_add_117(_lhs, _rhs)
        }),
    ));
    'switch: {
        match { (mode.read()) } {
            __v if __v == facebook_yoga_SizingMode_StretchFit
                || __v == facebook_yoga_SizingMode_FitContent =>
            {
                let __rhs =
                    if (({ facebook_yoga_FloatOptionalImpl::isUndefined(&maxSize.as_pointer()) })
                        || ({ (size.read()) } < {
                            ({ facebook_yoga_FloatOptionalImpl::unwrap(&maxSize.as_pointer()) })
                        }))
                    {
                        (size.read())
                    } else {
                        ({ facebook_yoga_FloatOptionalImpl::unwrap(&maxSize.as_pointer()) })
                    };
                size.write(__rhs);
                break 'switch;
            }
            __v if __v == facebook_yoga_SizingMode_MaxContent => {
                if ({ facebook_yoga_FloatOptionalImpl::isDefined(&maxSize.as_pointer()) }) {
                    mode.write(facebook_yoga_SizingMode_FitContent);
                    let __rhs =
                        ({ facebook_yoga_FloatOptionalImpl::unwrap(&maxSize.as_pointer()) });
                    size.write(__rhs);
                }
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn computeFlexBasisForChild_450(
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut width: f32,
    mut widthMode: facebook_yoga_SizingMode,
    mut height: f32,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut heightMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) {
    let mut mainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_130(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                )
            }),
            direction,
        )
    });
    let mut isMainAxisRow: bool = ({ isRow_128(mainAxis) });
    let mut mainAxisSize: f32 = if isMainAxisRow { width } else { height };
    let mut mainAxisOwnerSize: f32 = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let childWidth: Value<f32> = Rc::new(RefCell::new(YGUndefined_23.with(|rc| *rc.borrow())));
    let childHeight: Value<f32> = Rc::new(RefCell::new(YGUndefined_23.with(|rc| *rc.borrow())));
    let childWidthSizingMode: Value<facebook_yoga_SizingMode> =
        <Value<facebook_yoga_SizingMode>>::default();
    let childHeightSizingMode: Value<facebook_yoga_SizingMode> =
        <Value<facebook_yoga_SizingMode>>::default();
    let resolvedFlexBasis: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            facebook_yoga_NodeImpl::resolveFlexBasis(
                &child,
                direction,
                mainAxis,
                mainAxisOwnerSize,
                ownerWidth,
            )
        }),
    ));
    let mut isRowStyleDimDefined: bool = ({
        facebook_yoga_NodeImpl::hasDefiniteLength(&child, facebook_yoga_Dimension_Width, ownerWidth)
    });
    let mut isColumnStyleDimDefined: bool = ({
        facebook_yoga_NodeImpl::hasDefiniteLength(
            &child,
            facebook_yoga_Dimension_Height,
            ownerHeight,
        )
    });
    let mut fixFlexBasisFitContent: bool = ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    let mut useResolvedFlexBasis: bool =
        ({ facebook_yoga_FloatOptionalImpl::isDefined(&resolvedFlexBasis.as_pointer()) })
            && ({ isDefined_107(mainAxisSize) });
    if useResolvedFlexBasis {
        if ({
            facebook_yoga_FloatOptionalImpl::isUndefined(&field_ptr!(
                ({ facebook_yoga_NodeImpl::getLayout_26(&child,) }),
                computedFlexBasis
            ))
        }) || (({
            facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
                &({ facebook_yoga_NodeImpl::getConfig(&child) }),
                facebook_yoga_ExperimentalFeature_WebFlexBasis,
            )
        }) && ({
            ({ facebook_yoga_NodeImpl::getLayout_26(&child) })
                .with(|__s| __s.computedFlexBasisGeneration)
        } != { generationCount }))
        {
            let mut paddingAndBorder: facebook_yoga_FloatOptional =
                facebook_yoga_FloatOptional::new_1({
                    ({
                        paddingAndBorderForAxis_417(
                            (child).clone(),
                            mainAxis,
                            direction,
                            ownerWidth,
                        )
                    })
                });
            ({
                facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(
                    &child,
                    ({
                        maxOrDefined_121(
                            (*resolvedFlexBasis.borrow()).clone(),
                            (paddingAndBorder).clone(),
                        )
                    }),
                )
            });
        }
    } else if (isMainAxisRow) && (isRowStyleDimDefined) {
        let mut paddingAndBorder: facebook_yoga_FloatOptional =
            facebook_yoga_FloatOptional::new_1({
                ({
                    paddingAndBorderForAxis_417(
                        (child).clone(),
                        facebook_yoga_FlexDirection_Row,
                        direction,
                        ownerWidth,
                    )
                })
            });
        ({
            let _computedFlexBasis: facebook_yoga_FloatOptional = ({
                maxOrDefined_121(
                    ({
                        let _referenceLength: f32 = ownerWidth;
                        let _ownerWidth: f32 = ownerWidth;
                        facebook_yoga_NodeImpl::getResolvedDimension(
                            &child,
                            direction,
                            facebook_yoga_Dimension_Width,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                    (paddingAndBorder).clone(),
                )
            });
            facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(&child, _computedFlexBasis)
        });
    } else if (!(isMainAxisRow)) && (isColumnStyleDimDefined) {
        let mut paddingAndBorder: facebook_yoga_FloatOptional =
            facebook_yoga_FloatOptional::new_1({
                ({
                    paddingAndBorderForAxis_417(
                        (child).clone(),
                        facebook_yoga_FlexDirection_Column,
                        direction,
                        ownerWidth,
                    )
                })
            });
        ({
            let _computedFlexBasis: facebook_yoga_FloatOptional = ({
                maxOrDefined_121(
                    ({
                        facebook_yoga_NodeImpl::getResolvedDimension(
                            &child,
                            direction,
                            facebook_yoga_Dimension_Height,
                            ownerHeight,
                            ownerWidth,
                        )
                    }),
                    (paddingAndBorder).clone(),
                )
            });
            facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(&child, _computedFlexBasis)
        });
    } else {
        (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_MaxContent;
        (*childHeightSizingMode.borrow_mut()) = facebook_yoga_SizingMode_MaxContent;
        let mut marginRow: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let mut marginColumn: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        });
        if isRowStyleDimDefined {
            (*childWidth.borrow_mut()) = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _referenceLength: f32 = ownerWidth;
                                let _ownerWidth: f32 = ownerWidth;
                                facebook_yoga_NodeImpl::getResolvedDimension(
                                    &child,
                                    direction,
                                    facebook_yoga_Dimension_Width,
                                    _referenceLength,
                                    _ownerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } + { marginRow });
            (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
        }
        if isColumnStyleDimDefined {
            (*childHeight.borrow_mut()) = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                facebook_yoga_NodeImpl::getResolvedDimension(
                                    &child,
                                    direction,
                                    facebook_yoga_Dimension_Height,
                                    ownerHeight,
                                    ownerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } + { marginColumn });
            (*childHeightSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
        }
        if ((!(isMainAxisRow))
            && (({
                facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_25(&node) }))
            }) == facebook_yoga_Overflow_Scroll))
            || (({
                facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_25(&node) }))
            }) != facebook_yoga_Overflow_Scroll)
        {
            if ({ isUndefined_105((*childWidth.borrow())) }) && ({ isDefined_107(width) }) {
                (*childWidth.borrow_mut()) = width;
                (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_FitContent;
            }
        }
        let mut parentDoesNotScroll: bool = (!((node).is_null()))
            && (({
                facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_25(&node) }))
            }) != facebook_yoga_Overflow_Scroll);
        let mut applyHeightFitContent: bool = (isMainAxisRow) || (parentDoesNotScroll);
        if fixFlexBasisFitContent {
            let mut childHadOverflow: bool = ((!((child).is_null()))
                && ({ facebook_yoga_NodeImpl::isDirty(&child) }))
                && ({
                    facebook_yoga_LayoutResultsImpl::hadOverflow(
                        &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    )
                });
            let mut hasHeightIndependentSubtree: bool =
                ((((((((!((node).is_null())) && (!((child).is_null()))) && (!(isMainAxisRow)))
                    && (parentDoesNotScroll))
                    && ({ isUndefined_105((*childHeight.borrow())) }))
                    && ({ isDefined_107(height) }))
                    && ({ isColumnStretchEdge_441((node).clone(), (child).clone()) }))
                    && ({ isInColumnStretchScrollSubtree_442((node).clone()) }))
                    && ({ canSkipHeightFitContent_448((child).clone()) });
            if (hasHeightIndependentSubtree) && (childHadOverflow) {
                ({ facebook_yoga_NodeImpl::setLayoutHadOverflow(&child, false) });
            }
            if hasHeightIndependentSubtree {
                applyHeightFitContent = false;
            }
        }
        if ((applyHeightFitContent) && ({ isUndefined_105((*childHeight.borrow())) }))
            && ({ isDefined_107(height) })
        {
            (*childHeight.borrow_mut()) = height;
            (*childHeightSizingMode.borrow_mut()) = facebook_yoga_SizingMode_FitContent;
        }
        let childStyle: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&child) });
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                ))
                .as_pointer(),
            )
        }) {
            if (!(isMainAxisRow))
                && ((*childWidthSizingMode.borrow()) == facebook_yoga_SizingMode_StretchFit)
            {
                (*childHeight.borrow_mut()) = ({ marginColumn } + {
                    ({ ((*childWidth.borrow()) - marginRow) } / {
                        ({
                            facebook_yoga_FloatOptionalImpl::unwrap(
                                &Rc::new(RefCell::new(
                                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                                ))
                                .as_pointer(),
                            )
                        })
                    })
                });
                (*childHeightSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            } else if (isMainAxisRow)
                && ((*childHeightSizingMode.borrow()) == facebook_yoga_SizingMode_StretchFit)
            {
                (*childWidth.borrow_mut()) = ({ marginRow } + {
                    ({ ((*childHeight.borrow()) - marginColumn) } * {
                        ({
                            facebook_yoga_FloatOptionalImpl::unwrap(
                                &Rc::new(RefCell::new(
                                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                                ))
                                .as_pointer(),
                            )
                        })
                    })
                });
                (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            }
        }
        let mut hasExactWidth: bool =
            ({ isDefined_107(width) }) && (widthMode == facebook_yoga_SizingMode_StretchFit);
        let mut childWidthStretch: bool =
            (({ resolveChildAlignment_413((node).clone(), (child).clone()) })
                == facebook_yoga_Align_Stretch)
                && ((*childWidthSizingMode.borrow()) != facebook_yoga_SizingMode_StretchFit);
        if (((!(isMainAxisRow)) && (!(isRowStyleDimDefined))) && (hasExactWidth))
            && (childWidthStretch)
        {
            (*childWidth.borrow_mut()) = width;
            (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            if ({
                facebook_yoga_FloatOptionalImpl::isDefined(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                    ))
                    .as_pointer(),
                )
            }) {
                (*childHeight.borrow_mut()) = ({ ((*childWidth.borrow()) - marginRow) } / {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                            ))
                            .as_pointer(),
                        )
                    })
                });
                (*childHeightSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            }
        }
        let mut hasExactHeight: bool =
            ({ isDefined_107(height) }) && (heightMode == facebook_yoga_SizingMode_StretchFit);
        let mut childHeightStretch: bool =
            (({ resolveChildAlignment_413((node).clone(), (child).clone()) })
                == facebook_yoga_Align_Stretch)
                && ((*childHeightSizingMode.borrow()) != facebook_yoga_SizingMode_StretchFit);
        if (((isMainAxisRow) && (!(isColumnStyleDimDefined))) && (hasExactHeight))
            && (childHeightStretch)
        {
            (*childHeight.borrow_mut()) = height;
            (*childHeightSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            if ({
                facebook_yoga_FloatOptionalImpl::isDefined(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                    ))
                    .as_pointer(),
                )
            }) {
                (*childWidth.borrow_mut()) = ({ ((*childHeight.borrow()) - marginColumn) } * {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                            ))
                            .as_pointer(),
                        )
                    })
                });
                (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            }
        }
        ({
            let _ownerAxisSize: f32 = ownerWidth;
            let _ownerWidth: f32 = ownerWidth;
            constrainMaxSizeForMode_449(
                (child).clone(),
                direction,
                facebook_yoga_FlexDirection_Row,
                _ownerAxisSize,
                _ownerWidth,
                (childWidthSizingMode.as_pointer()),
                (childWidth.as_pointer()),
            )
        });
        ({
            constrainMaxSizeForMode_449(
                (child).clone(),
                direction,
                facebook_yoga_FlexDirection_Column,
                ownerHeight,
                ownerWidth,
                (childHeightSizingMode.as_pointer()),
                (childHeight.as_pointer()),
            )
        });
        ({
            let _node: Ptr<facebook_yoga_Node> = (child).clone();
            let _availableWidth: f32 = (*childWidth.borrow());
            let _availableHeight: f32 = (*childHeight.borrow());
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = (*childWidthSizingMode.borrow());
            let _heightSizingMode: facebook_yoga_SizingMode = (*childHeightSizingMode.borrow());
            let _ownerWidth: f32 = ownerWidth;
            let _ownerHeight: f32 = ownerHeight;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutInternal_430(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                false,
                facebook_yoga_LayoutPassReason_kMeasureChild,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        ({
            let _computedFlexBasis: facebook_yoga_FloatOptional =
                facebook_yoga_FloatOptional::new_1({
                    ({
                        let _a: f32 = ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                ({ dimension_136(mainAxis) }),
                            )
                        });
                        let _b: f32 = ({
                            paddingAndBorderForAxis_417(
                                (child).clone(),
                                mainAxis,
                                direction,
                                ownerWidth,
                            )
                        });
                        maxOrDefined_110(_a, _b)
                    })
                });
            facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(&child, _computedFlexBasis)
        });
    }
    ({ facebook_yoga_NodeImpl::setLayoutComputedFlexBasisGeneration(&child, generationCount) });
}
pub fn measureNodeWithMeasureFunc_451(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut availableWidth: f32,
    mut availableHeight: f32,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut reason: facebook_yoga_LayoutPassReason,
) {
    ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _condition: bool = ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) });
        assertFatalWithNode_213(
            _node,
            _condition,
            Ptr::<i8>::from_string_literal(b"Expected node to have custom measure function"),
        )
    });
    if (widthSizingMode == facebook_yoga_SizingMode_MaxContent) {
        availableWidth = YGUndefined_23.with(|rc| *rc.borrow());
    }
    if (heightSizingMode == facebook_yoga_SizingMode_MaxContent) {
        availableHeight = YGUndefined_23.with(|rc| *rc.borrow());
    }
    let layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    let mut paddingAndBorderAxisRow: f32 = ({
        ({
            (({
                facebook_yoga_LayoutResultsImpl::padding(&layout, facebook_yoga_PhysicalEdge_Left)
            }) + ({
                facebook_yoga_LayoutResultsImpl::padding(&layout, facebook_yoga_PhysicalEdge_Right)
            }))
        } + {
            ({ facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Left) })
        })
    } + {
        ({ facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Right) })
    });
    let mut paddingAndBorderAxisColumn: f32 = ({
        ({
            (({
                facebook_yoga_LayoutResultsImpl::padding(&layout, facebook_yoga_PhysicalEdge_Top)
            }) + ({
                facebook_yoga_LayoutResultsImpl::padding(&layout, facebook_yoga_PhysicalEdge_Bottom)
            }))
        } + {
            ({ facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Top) })
        })
    } + {
        ({ facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Bottom) })
    });
    let mut innerWidth: f32 = if ({ isUndefined_105(availableWidth) }) {
        availableWidth
    } else {
        ({ maxOrDefined_110(0_f32, (availableWidth - paddingAndBorderAxisRow)) })
    };
    let mut innerHeight: f32 = if ({ isUndefined_105(availableHeight) }) {
        availableHeight
    } else {
        ({ maxOrDefined_110(0_f32, (availableHeight - paddingAndBorderAxisColumn)) })
    };
    if (widthSizingMode == facebook_yoga_SizingMode_StretchFit)
        && (heightSizingMode == facebook_yoga_SizingMode_StretchFit)
    {
        ({
            let _measuredDimension: f32 = ({
                let _axisSize: f32 = ownerWidth;
                let _widthSize: f32 = ownerWidth;
                boundAxis_419(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    availableWidth,
                    _axisSize,
                    _widthSize,
                )
            });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                facebook_yoga_Dimension_Width,
            )
        });
        ({
            let _measuredDimension: f32 = ({
                boundAxis_419(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    availableHeight,
                    ownerHeight,
                    ownerWidth,
                )
            });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                facebook_yoga_Dimension_Height,
            )
        });
    } else {
        ({ facebook_yoga_Event::publish_3_5(node.reinterpret_cast::<YGNode>(), None) });
        let mut measuredSize: YGSize = ({
            let _widthMode: facebook_yoga_MeasureMode = ({ measureMode_102(widthSizingMode) });
            let _heightMode: facebook_yoga_MeasureMode = ({ measureMode_102(heightSizingMode) });
            facebook_yoga_NodeImpl::measure(&node, innerWidth, _widthMode, innerHeight, _heightMode)
        });
        {
            field!(layoutMarkerData, measureCallbacks).with_mut(|__v| *__v = *__v + 1)
        };
        {
            field_ref!(layoutMarkerData, measureCallbackReasonsCount)
                .decay()
                .offset((reason as usize) as isize)
                .with_mut(|__v| *__v = *__v + 1)
        };
        ({
            let _eventData: Value<
                facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_,
            > = Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_ {
                    width: innerWidth,
                    widthMeasureMode: ({ unscopedEnum_95(({ measureMode_102(widthSizingMode) })) }),
                    height: innerHeight,
                    heightMeasureMode: ({
                        unscopedEnum_95(({ measureMode_102(heightSizingMode) }))
                    }),
                    measuredWidth: measuredSize.width,
                    measuredHeight: measuredSize.height,
                    reason: reason,
                },
            ));
            facebook_yoga_Event::publish_3_6(
                node.reinterpret_cast::<YGNode>(),
                _eventData.as_pointer(),
            )
        });
        ({
            let _measuredDimension: f32 = ({
                let _axisSize: f32 = ownerWidth;
                let _widthSize: f32 = ownerWidth;
                boundAxis_419(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    if ((widthSizingMode == facebook_yoga_SizingMode_MaxContent)
                        || (widthSizingMode == facebook_yoga_SizingMode_FitContent))
                    {
                        (measuredSize.width + paddingAndBorderAxisRow)
                    } else {
                        availableWidth
                    },
                    _axisSize,
                    _widthSize,
                )
            });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                facebook_yoga_Dimension_Width,
            )
        });
        ({
            let _measuredDimension: f32 = ({
                boundAxis_419(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    if ((heightSizingMode == facebook_yoga_SizingMode_MaxContent)
                        || (heightSizingMode == facebook_yoga_SizingMode_FitContent))
                    {
                        (measuredSize.height + paddingAndBorderAxisColumn)
                    } else {
                        availableHeight
                    },
                    ownerHeight,
                    ownerWidth,
                )
            });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                facebook_yoga_Dimension_Height,
            )
        });
    }
}
pub fn measureNodeWithoutChildren_452(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut availableWidth: f32,
    mut availableHeight: f32,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
) {
    let layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    let mut width: f32 = availableWidth;
    if (widthSizingMode == facebook_yoga_SizingMode_MaxContent)
        || (widthSizingMode == facebook_yoga_SizingMode_FitContent)
    {
        width = ({
            ({
                (({
                    facebook_yoga_LayoutResultsImpl::padding(
                        &layout,
                        facebook_yoga_PhysicalEdge_Left,
                    )
                }) + ({
                    facebook_yoga_LayoutResultsImpl::padding(
                        &layout,
                        facebook_yoga_PhysicalEdge_Right,
                    )
                }))
            } + {
                ({
                    facebook_yoga_LayoutResultsImpl::border(
                        &layout,
                        facebook_yoga_PhysicalEdge_Left,
                    )
                })
            })
        } + {
            ({ facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Right) })
        });
    }
    ({
        let _measuredDimension: f32 = ({
            let _axisSize: f32 = ownerWidth;
            let _widthSize: f32 = ownerWidth;
            boundAxis_419(
                (node).clone(),
                facebook_yoga_FlexDirection_Row,
                direction,
                width,
                _axisSize,
                _widthSize,
            )
        });
        facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
            &node,
            _measuredDimension,
            facebook_yoga_Dimension_Width,
        )
    });
    let mut height: f32 = availableHeight;
    if (heightSizingMode == facebook_yoga_SizingMode_MaxContent)
        || (heightSizingMode == facebook_yoga_SizingMode_FitContent)
    {
        height = ({
            ({
                (({
                    facebook_yoga_LayoutResultsImpl::padding(
                        &layout,
                        facebook_yoga_PhysicalEdge_Top,
                    )
                }) + ({
                    facebook_yoga_LayoutResultsImpl::padding(
                        &layout,
                        facebook_yoga_PhysicalEdge_Bottom,
                    )
                }))
            } + {
                ({
                    facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Top)
                })
            })
        } + {
            ({
                facebook_yoga_LayoutResultsImpl::border(&layout, facebook_yoga_PhysicalEdge_Bottom)
            })
        });
    }
    ({
        let _measuredDimension: f32 = ({
            boundAxis_419(
                (node).clone(),
                facebook_yoga_FlexDirection_Column,
                direction,
                height,
                ownerHeight,
                ownerWidth,
            )
        });
        facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
            &node,
            _measuredDimension,
            facebook_yoga_Dimension_Height,
        )
    });
}
pub fn isFixedSize_453(mut dim: f32, mut sizingMode: facebook_yoga_SizingMode) -> bool {
    return (sizingMode == facebook_yoga_SizingMode_StretchFit)
        || ((({ isDefined_107(dim) }) && (sizingMode == facebook_yoga_SizingMode_FitContent))
            && ((dim as f64) <= 0_f64));
}
pub fn measureNodeWithFixedSize_454(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut availableWidth: f32,
    mut availableHeight: f32,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
) -> bool {
    if ({ isFixedSize_453(availableWidth, widthSizingMode) })
        && ({ isFixedSize_453(availableHeight, heightSizingMode) })
    {
        ({
            let _measuredDimension: f32 = ({
                let _axisSize: f32 = ownerWidth;
                let _widthSize: f32 = ownerWidth;
                boundAxis_419(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    if ({ isUndefined_105(availableWidth) })
                        || ((widthSizingMode == facebook_yoga_SizingMode_FitContent)
                            && (availableWidth < 0_f32))
                    {
                        0_f32
                    } else {
                        availableWidth
                    },
                    _axisSize,
                    _widthSize,
                )
            });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                facebook_yoga_Dimension_Width,
            )
        });
        ({
            let _measuredDimension: f32 = ({
                boundAxis_419(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    if ({ isUndefined_105(availableHeight) })
                        || ((heightSizingMode == facebook_yoga_SizingMode_FitContent)
                            && (availableHeight < 0_f32))
                    {
                        0_f32
                    } else {
                        availableHeight
                    },
                    ownerHeight,
                    ownerWidth,
                )
            });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                facebook_yoga_Dimension_Height,
            )
        });
        return true;
    }
    return false;
}
pub fn zeroOutLayoutRecursively_455(mut node: Ptr<facebook_yoga_Node>) {
    let __rhs = <facebook_yoga_LayoutResults>::default();
    ({ facebook_yoga_NodeImpl::getLayout_26(&node) }).write(__rhs);
    ({ facebook_yoga_NodeImpl::setLayoutDimension(&node, 0_f32, facebook_yoga_Dimension_Width) });
    ({ facebook_yoga_NodeImpl::setLayoutDimension(&node, 0_f32, facebook_yoga_Dimension_Height) });
    ({ facebook_yoga_NodeImpl::setHasNewLayout(&node, true) });
    ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&node) });
    let __range2: Ptr<Vec<Ptr<facebook_yoga_Node>>> =
        ({ facebook_yoga_NodeImpl::getChildren(&node) });
    'loop_: for child in IterRange::new(__range2.decay(), __range2.decay().to_end()) {
        let mut child: Ptr<facebook_yoga_Node> = (child.read());
        ({ zeroOutLayoutRecursively_455((child).clone()) });
    }
}
pub fn cleanupContentsNodesRecursively_432(
    mut node: Ptr<facebook_yoga_Node>,
    mut didPerformLayout: bool,
) {
    if ({ facebook_yoga_NodeImpl::hasContentsChildren(&node) }) {
        {
            ({ facebook_yoga_NodeImpl::cloneContentsChildrenIfNeeded(&node) });
            let __range3: Ptr<Vec<Ptr<facebook_yoga_Node>>> =
                ({ facebook_yoga_NodeImpl::getChildren(&node) });
            'loop_: for child in IterRange::new(__range3.decay(), __range3.decay().to_end()) {
                let mut child: Ptr<facebook_yoga_Node> = (child.read());
                if (({
                    facebook_yoga_StyleImpl::display(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    )
                }) == facebook_yoga_Display_Contents)
                {
                    let __rhs = <facebook_yoga_LayoutResults>::default();
                    ({ facebook_yoga_NodeImpl::getLayout_26(&child) }).write(__rhs);
                    ({
                        facebook_yoga_NodeImpl::setLayoutDimension(
                            &child,
                            0_f32,
                            facebook_yoga_Dimension_Width,
                        )
                    });
                    ({
                        facebook_yoga_NodeImpl::setLayoutDimension(
                            &child,
                            0_f32,
                            facebook_yoga_Dimension_Height,
                        )
                    });
                    if didPerformLayout {
                        ({ facebook_yoga_NodeImpl::setHasNewLayout(&child, true) });
                    }
                    ({ facebook_yoga_NodeImpl::setDirty(&child, false) });
                    ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&child) });
                    ({ cleanupContentsNodesRecursively_432((child).clone(), didPerformLayout) });
                }
            }
        };
    }
}
pub fn calculateAvailableInnerDimension_456(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut dimension: facebook_yoga_Dimension,
    mut availableDim: f32,
    mut paddingAndBorder: f32,
    mut ownerDim: f32,
    mut ownerWidth: f32,
) -> f32 {
    let mut availableInnerDim: f32 = (availableDim - paddingAndBorder);
    if ({ isDefined_107(availableInnerDim) }) {
        let minDimensionOptional: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
            ({
                facebook_yoga_StyleImpl::resolvedMinDimension(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    direction,
                    dimension,
                    ownerDim,
                    ownerWidth,
                )
            }),
        ));
        let mut minInnerDim: f32 = if ({
            facebook_yoga_FloatOptionalImpl::isUndefined(&minDimensionOptional.as_pointer())
        }) {
            0_f32
        } else {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&minDimensionOptional.as_pointer()) })
                - paddingAndBorder)
        };
        let maxDimensionOptional: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
            ({
                facebook_yoga_StyleImpl::resolvedMaxDimension(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    direction,
                    dimension,
                    ownerDim,
                    ownerWidth,
                )
            }),
        ));
        let mut maxInnerDim: f32 = if ({
            facebook_yoga_FloatOptionalImpl::isUndefined(&maxDimensionOptional.as_pointer())
        }) {
            3.4028235e+38_f32
        } else {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&maxDimensionOptional.as_pointer()) })
                - paddingAndBorder)
        };
        let __rhs = ({
            maxOrDefined_110(
                ({ minOrDefined_438(availableInnerDim, maxInnerDim) }),
                minInnerDim,
            )
        });
        availableInnerDim = __rhs;
    }
    return availableInnerDim;
}
pub fn computeFlexBasisForChildren_457(
    mut node: Ptr<facebook_yoga_Node>,
    mut availableInnerWidth: f32,
    mut availableInnerHeight: f32,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut performLayout: bool,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) -> f32 {
    let mut totalOuterFlexBasis: f32 = 0_f32;
    let mut singleFlexChild: Ptr<YGNode> = Ptr::<YGNode>::null();
    let children: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
        RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
    );
    let mut sizingModeMainDim: facebook_yoga_SizingMode = if ({ isRow_128(mainAxis) }) {
        widthSizingMode
    } else {
        heightSizingMode
    };
    if (sizingModeMainDim == facebook_yoga_SizingMode_StretchFit) {
        let __begin3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(
                        &children.as_pointer(),
                    )
                }),
            ));
        let __end3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(
                        &children.as_pointer(),
                    )
                }),
            ));
        'loop_: while ({
            let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                __begin3.as_pointer();
            operator_ne_78(_a, __end3.as_pointer())
        }) {
            let mut child: Ptr<facebook_yoga_Node> = ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                    &__begin3.as_pointer(),
                )
            });
            if ({ facebook_yoga_NodeImpl::isNodeFlexible(&child) }) {
                if ((!((singleFlexChild).is_null()))
                    || ({
                        inexactEquals_111(
                            ({ facebook_yoga_NodeImpl::resolveFlexGrow(&child) }),
                            0_f32,
                        )
                    }))
                    || ({
                        inexactEquals_111(
                            ({ facebook_yoga_NodeImpl::resolveFlexShrink(&child) }),
                            0_f32,
                        )
                    })
                {
                    singleFlexChild = Ptr::<YGNode>::null();
                    break;
                } else {
                    singleFlexChild = child.reinterpret_cast::<YGNode>();
                }
            }
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin3.as_pointer(),
                )
            });
        }
    };
    let __begin2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(
                    &children.as_pointer(),
                )
            }),
        ));
    let __end2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(
                    &children.as_pointer(),
                )
            }),
        ));
    'loop_: while ({
        let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            __begin2.as_pointer();
        operator_ne_78(_a, __end2.as_pointer())
    }) {
        let mut child: Ptr<facebook_yoga_Node> = ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                &__begin2.as_pointer(),
            )
        });
        ({ facebook_yoga_NodeImpl::processDimensions(&child) });
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
            == facebook_yoga_Display_None)
        {
            if performLayout {
                ({ zeroOutLayoutRecursively_455((child).clone()) });
                ({ facebook_yoga_NodeImpl::setHasNewLayout(&child, true) });
                ({ facebook_yoga_NodeImpl::setDirty(&child, false) });
            }
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin2.as_pointer(),
                )
            });
            continue 'loop_;
        }
        if performLayout {
            let mut childDirection: facebook_yoga_Direction =
                ({ facebook_yoga_NodeImpl::resolveDirection(&child, direction) });
            ({
                facebook_yoga_NodeImpl::setPosition(
                    &child,
                    childDirection,
                    availableInnerWidth,
                    availableInnerHeight,
                )
            });
        }
        if (({
            facebook_yoga_StyleImpl::positionType(&({ facebook_yoga_NodeImpl::style_24(&child) }))
        }) == facebook_yoga_PositionType_Absolute)
        {
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin2.as_pointer(),
                )
            });
            continue 'loop_;
        }
        if ({ child.reinterpret_cast::<YGNode>() } == { (singleFlexChild).clone() }) {
            ({
                facebook_yoga_NodeImpl::setLayoutComputedFlexBasisGeneration(
                    &child,
                    generationCount,
                )
            });
            ({
                facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(
                    &child,
                    facebook_yoga_FloatOptional::new_1({ 0_f32 }),
                )
            });
        } else {
            ({
                let _node: Ptr<facebook_yoga_Node> = (node).clone();
                let _child: Ptr<facebook_yoga_Node> = (child).clone();
                let _width: f32 = availableInnerWidth;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _height: f32 = availableInnerHeight;
                let _ownerWidth: f32 = ownerWidth;
                let _ownerHeight: f32 = ownerHeight;
                let _heightMode: facebook_yoga_SizingMode = heightSizingMode;
                let _direction: facebook_yoga_Direction = direction;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _depth: u32 = depth;
                let _generationCount: u32 = generationCount;
                computeFlexBasisForChild_450(
                    _node,
                    _child,
                    _width,
                    _widthMode,
                    _height,
                    _ownerWidth,
                    _ownerHeight,
                    _heightMode,
                    _direction,
                    _layoutMarkerData,
                    _depth,
                    _generationCount,
                )
            });
        }
        totalOuterFlexBasis += ({
            ({
                facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!(
                    ({ facebook_yoga_NodeImpl::getLayout_26(&child,) }),
                    computedFlexBasis
                ))
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeMarginForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    mainAxis,
                    availableInnerWidth,
                )
            })
        });
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                &__begin2.as_pointer(),
            )
        });
    }
    return totalOuterFlexBasis;
}
pub fn computeMinContentMainSize_458(
    mut node: Ptr<facebook_yoga_Node>,
    mut requestedAxis: facebook_yoga_FlexDirection,
    mut ownerDirection: facebook_yoga_Direction,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut generationCount: u32,
) -> f32 {
    let mut wantRow: bool = ({ isRow_128(requestedAxis) });
    let staticMin: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(if wantRow {
        ({ facebook_yoga_NodeImpl::getMinContentWidth(&node) })
    } else {
        ({ facebook_yoga_NodeImpl::getMinContentHeight(&node) })
    }));
    if ({ facebook_yoga_FloatOptionalImpl::isDefined(&staticMin.as_pointer()) }) {
        return ({ facebook_yoga_FloatOptionalImpl::unwrap(&staticMin.as_pointer()) });
    }
    let layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    let mut axisIndex: u8 = ({ to_underlying_47(({ dimension_136(requestedAxis) })) });
    if ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) }) {
        if ({
            (field_ref!(layout, minContentGeneration)
                .decay()
                .offset((axisIndex as usize) as isize)
                .read())
        } != { generationCount })
        {
            let mut size: YGSize = if ({ facebook_yoga_NodeImpl::hasMinContentMeasureFunc(&node) })
            {
                ({
                    let _availableWidth: f32 = if wantRow {
                        0_f32
                    } else {
                        YGUndefined_23.with(|rc| *rc.borrow())
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: f32 = if wantRow {
                        YGUndefined_23.with(|rc| *rc.borrow())
                    } else {
                        0_f32
                    };
                    let _heightMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_Undefined
                    } else {
                        facebook_yoga_MeasureMode_AtMost
                    };
                    facebook_yoga_NodeImpl::measureMinContent(
                        &node,
                        _availableWidth,
                        _widthMode,
                        _availableHeight,
                        _heightMode,
                    )
                })
            } else {
                ({
                    let _availableWidth: f32 = if wantRow {
                        0_f32
                    } else {
                        YGUndefined_23.with(|rc| *rc.borrow())
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: f32 = if wantRow {
                        YGUndefined_23.with(|rc| *rc.borrow())
                    } else {
                        0_f32
                    };
                    let _heightMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_Undefined
                    } else {
                        facebook_yoga_MeasureMode_AtMost
                    };
                    facebook_yoga_NodeImpl::measure(
                        &node,
                        _availableWidth,
                        _widthMode,
                        _availableHeight,
                        _heightMode,
                    )
                })
            };
            field_ref!(layout, minContent)
                .decay()
                .offset((axisIndex as usize) as isize)
                .write(if wantRow { size.width } else { size.height });
            field_ref!(layout, minContentGeneration)
                .decay()
                .offset((axisIndex as usize) as isize)
                .write(generationCount);
        }
        let mut leafDirection: facebook_yoga_Direction =
            ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
        let mut paddingAndBorder: f32 = ({
            ({
                facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    requestedAxis,
                    leafDirection,
                    ownerWidth,
                )
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeFlexEndPaddingAndBorder(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    requestedAxis,
                    leafDirection,
                    ownerWidth,
                )
            })
        });
        return ({
            (field_ref!(layout, minContent)
                .decay()
                .offset((axisIndex as usize) as isize)
                .read())
        } + { paddingAndBorder });
    }
    if (({ facebook_yoga_NodeImpl::getChildCount(&node) }) == 0_usize) {
        return 0_f32;
    }
    if ({
        (field_ref!(layout, minContentGeneration)
            .decay()
            .offset((axisIndex as usize) as isize)
            .read())
    } == { generationCount })
        && ({
            inexactEquals_111(
                (field_ref!(layout, minContentOwnerWidth)
                    .decay()
                    .offset((axisIndex as usize) as isize)
                    .read()),
                ownerWidth,
            )
        })
    {
        return (field_ref!(layout, minContent)
            .decay()
            .offset((axisIndex as usize) as isize)
            .read());
    }
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    let mut nodeMainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_130(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            }),
            direction,
        )
    });
    let mut nodeCrossAxis: facebook_yoga_FlexDirection =
        ({ resolveCrossDirection_131(nodeMainAxis, direction) });
    let mut mainTotal: f32 = 0_f32;
    let crossMax: Value<f32> = Rc::new(RefCell::new(0_f32));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { ({ facebook_yoga_NodeImpl::getChildCount(&node) }) }) {
        let mut child: Ptr<facebook_yoga_Node> = ({ facebook_yoga_NodeImpl::getChild(&node, i) });
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
            == facebook_yoga_Display_None)
            || (({
                facebook_yoga_StyleImpl::positionType(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                )
            }) == facebook_yoga_PositionType_Absolute)
        {
            i.postfix_inc();
            continue 'loop_;
        }
        let mut childMain: f32 = ({
            computeMinContentMainSize_458(
                (child).clone(),
                nodeMainAxis,
                direction,
                ownerWidth,
                ownerHeight,
                generationCount,
            )
        });
        childMain += ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                nodeMainAxis,
                ownerWidth,
            )
        });
        let childCross: Value<f32> = Rc::new(RefCell::new(
            ({
                computeMinContentMainSize_458(
                    (child).clone(),
                    nodeCrossAxis,
                    direction,
                    ownerWidth,
                    ownerHeight,
                    generationCount,
                )
            }),
        ));
        (*childCross.borrow_mut()) += ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                nodeCrossAxis,
                ownerWidth,
            )
        });
        mainTotal += childMain;
        let __rhs = ({
            let __a = crossMax.as_pointer();
            let __b = childCross.as_pointer();
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read());
        (*crossMax.borrow_mut()) = __rhs;
        i.postfix_inc();
    }
    mainTotal += ({
        ({
            facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                nodeMainAxis,
                direction,
                ownerWidth,
            )
        })
    } + {
        ({
            facebook_yoga_StyleImpl::computeFlexEndPaddingAndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                nodeMainAxis,
                direction,
                ownerWidth,
            )
        })
    });
    (*crossMax.borrow_mut()) += ({
        ({
            facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                nodeCrossAxis,
                direction,
                ownerWidth,
            )
        })
    } + {
        ({
            facebook_yoga_StyleImpl::computeFlexEndPaddingAndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                nodeCrossAxis,
                direction,
                ownerWidth,
            )
        })
    });
    let mut nodeMainIsRow: bool = ({ isRow_128(nodeMainAxis) });
    let mut widthMin: f32 = if nodeMainIsRow {
        mainTotal
    } else {
        (*crossMax.borrow())
    };
    let mut heightMin: f32 = if nodeMainIsRow {
        (*crossMax.borrow())
    } else {
        mainTotal
    };
    let mut minContent: f32 = if wantRow { widthMin } else { heightMin };
    field_ref!(layout, minContent)
        .decay()
        .offset((axisIndex as usize) as isize)
        .write(minContent);
    field_ref!(layout, minContentOwnerWidth)
        .decay()
        .offset((axisIndex as usize) as isize)
        .write(ownerWidth);
    field_ref!(layout, minContentGeneration)
        .decay()
        .offset((axisIndex as usize) as isize)
        .write(generationCount);
    return minContent;
}
pub fn computeAutoMinMainSize_459(
    mut child: Ptr<facebook_yoga_Node>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut ownerMainAxisSize: f32,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut generationCount: u32,
) -> facebook_yoga_FloatOptional {
    if ({
        facebook_yoga_NodeImpl::hasErrata(
            &child,
            facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
        )
    }) {
        return <facebook_yoga_FloatOptional>::default();
    }
    if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
        == facebook_yoga_Display_None)
    {
        return <facebook_yoga_FloatOptional>::default();
    }
    if ({
        facebook_yoga_StyleSizeLengthImpl::isDefined(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::minDimension(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        ({ dimension_136(mainAxis) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    }) {
        return <facebook_yoga_FloatOptional>::default();
    }
    if (({ facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
        != facebook_yoga_Overflow_Visible)
    {
        return facebook_yoga_FloatOptional::new_1({ 0_f32 });
    }
    let mut mainDim: facebook_yoga_Dimension = ({ dimension_136(mainAxis) });
    let mut crossDim: facebook_yoga_Dimension = if ({ isRow_128(mainAxis) }) {
        facebook_yoga_Dimension_Height
    } else {
        facebook_yoga_Dimension_Width
    };
    let mut isMainAxisRow: bool = ({ isRow_128(mainAxis) });
    let specifiedMain: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            facebook_yoga_NodeImpl::getResolvedDimension(
                &child,
                direction,
                mainDim,
                ownerMainAxisSize,
                ownerWidth,
            )
        }),
    ));
    let transferredMain: Value<facebook_yoga_FloatOptional> =
        Rc::new(RefCell::new(<facebook_yoga_FloatOptional>::default()));
    let aspectRatio: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::aspectRatio(&({ facebook_yoga_NodeImpl::style_24(&child) })) }),
    ));
    if ({ facebook_yoga_FloatOptionalImpl::isDefined(&aspectRatio.as_pointer()) }) {
        let mut crossOwner: f32 = if isMainAxisRow {
            ownerHeight
        } else {
            ownerWidth
        };
        let crossResolved: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
            ({
                facebook_yoga_NodeImpl::getResolvedDimension(
                    &child, direction, crossDim, crossOwner, ownerWidth,
                )
            }),
        ));
        if ({ facebook_yoga_FloatOptionalImpl::isDefined(&crossResolved.as_pointer()) }) {
            let mut ratio: f32 =
                ({ facebook_yoga_FloatOptionalImpl::unwrap(&aspectRatio.as_pointer()) });
            let mut crossValue: f32 =
                ({ facebook_yoga_FloatOptionalImpl::unwrap(&crossResolved.as_pointer()) });
            (*transferredMain.borrow_mut()) = facebook_yoga_FloatOptional::new_1({
                if isMainAxisRow {
                    (crossValue * ratio)
                } else {
                    (crossValue / ratio)
                }
            });
        }
    }
    let mut contentMain: facebook_yoga_FloatOptional = facebook_yoga_FloatOptional::new_1({
        ({
            computeMinContentMainSize_458(
                (child).clone(),
                mainAxis,
                direction,
                ownerWidth,
                ownerHeight,
                generationCount,
            )
        })
    });
    let floor: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new((contentMain).clone()));
    if ({ facebook_yoga_FloatOptionalImpl::isDefined(&specifiedMain.as_pointer()) }) {
        if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&floor.as_pointer()) })
            || ({
                let _lhs: facebook_yoga_FloatOptional = (*specifiedMain.borrow()).clone();
                operator_lt_113(_lhs, (*floor.borrow()).clone())
            })
        {
            (*floor.borrow_mut()) = (*specifiedMain.borrow()).clone();
        }
    } else if ({ facebook_yoga_FloatOptionalImpl::isDefined(&transferredMain.as_pointer()) }) {
        if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&floor.as_pointer()) })
            || ({
                let _lhs: facebook_yoga_FloatOptional = (*transferredMain.borrow()).clone();
                operator_lt_113(_lhs, (*floor.borrow()).clone())
            })
        {
            (*floor.borrow_mut()) = (*transferredMain.borrow()).clone();
        }
    }
    let maxMain: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            facebook_yoga_StyleImpl::resolvedMaxDimension(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                direction,
                mainDim,
                ownerMainAxisSize,
                ownerWidth,
            )
        }),
    ));
    if ({ facebook_yoga_FloatOptionalImpl::isDefined(&maxMain.as_pointer()) })
        && ({
            let _lhs: facebook_yoga_FloatOptional = (*floor.borrow()).clone();
            operator_gt_118(_lhs, (*maxMain.borrow()).clone())
        })
    {
        (*floor.borrow_mut()) = (*maxMain.borrow()).clone();
    }
    if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&floor.as_pointer()) })
        || (({ facebook_yoga_FloatOptionalImpl::unwrap(&floor.as_pointer()) }) < 0_f32)
    {
        (*floor.borrow_mut()) = facebook_yoga_FloatOptional::new_1({ 0_f32 });
    }
    return (*floor.borrow()).clone();
}
pub fn boundAxisWithAutoMin_460(
    mut child: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut value: f32,
    mut axisSize: f32,
    mut widthSize: f32,
) -> f32 {
    let mut bounded: f32 =
        ({ boundAxis_419((child).clone(), axis, direction, value, axisSize, widthSize) });
    let autoMin: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({ facebook_yoga_NodeImpl::getLayout_27(&child) })
            .with(|__s| __s.computedAutoMinMainSize.clone()),
    ));
    if ({ facebook_yoga_FloatOptionalImpl::isDefined(&autoMin.as_pointer()) })
        && (bounded < ({ facebook_yoga_FloatOptionalImpl::unwrap(&autoMin.as_pointer()) }))
    {
        bounded = ({ facebook_yoga_FloatOptionalImpl::unwrap(&autoMin.as_pointer()) });
    }
    return bounded;
}
pub fn distributeFreeSpaceSecondPass_461(
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut node: Ptr<facebook_yoga_Node>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut ownerWidth: f32,
    mut mainAxisOwnerSize: f32,
    mut availableInnerMainDim: f32,
    mut availableInnerCrossDim: f32,
    mut availableInnerWidth: f32,
    mut availableInnerHeight: f32,
    mut mainAxisOverflows: bool,
    mut sizingModeCrossDim: facebook_yoga_SizingMode,
    mut performLayout: bool,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) -> f32 {
    let mut childFlexBasis: f32 = 0_f32;
    let mut flexShrinkScaledFactor: f32 = 0_f32;
    let mut flexGrowFactor: f32 = 0_f32;
    let mut deltaFreeSpace: f32 = 0_f32;
    let mut isMainAxisRow: bool = ({ isRow_128(mainAxis) });
    let mut isNodeFlexWrap: bool =
        (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            != facebook_yoga_Wrap_NoWrap);
    'loop_: for currentLineChild in IterRange::new(
        field_ref!(flexLine, itemsInFlow).decay(),
        field_ref!(flexLine, itemsInFlow).decay().to_end(),
    ) {
        let mut currentLineChild: Ptr<facebook_yoga_Node> = (currentLineChild.read());
        childFlexBasis = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _node: Ptr<facebook_yoga_Node> = (currentLineChild).clone();
                        let _value: facebook_yoga_FloatOptional =
                            ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild) })
                                .with(|__s| __s.computedFlexBasis.clone());
                        boundAxisWithinMinAndMax_418(
                            _node,
                            direction,
                            mainAxis,
                            _value,
                            mainAxisOwnerSize,
                            ownerWidth,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
        let mut updatedMainSize: f32 = childFlexBasis;
        if ({ isDefined_107(flexLine.with(|__s| __s.layout.remainingFreeSpace)) })
            && (flexLine.with(|__s| __s.layout.remainingFreeSpace) < 0_f32)
        {
            flexShrinkScaledFactor = ({
                -({ facebook_yoga_NodeImpl::resolveFlexShrink(&currentLineChild) })
            } * { childFlexBasis });
            if (flexShrinkScaledFactor != 0_f32) {
                let mut childSize: f32 = YGUndefined_23.with(|rc| *rc.borrow());
                let mut shrinkFactorMagnitude: f32 = flexLine
                    .with(|__s| __s.layout.totalFlexShrinkScaledFactors)
                    .abs();
                if ({ isDefined_107(flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors)) })
                    && ((shrinkFactorMagnitude < 1e-06_f32)
                        || (flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors) > 0_f32))
                {
                    childSize = (childFlexBasis + flexShrinkScaledFactor);
                } else {
                    childSize = ({ childFlexBasis } + {
                        ({
                            ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                                flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors)
                            })
                        } * { flexShrinkScaledFactor })
                    });
                }
                updatedMainSize = ({
                    boundAxisWithAutoMin_460(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        childSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
            }
        } else if ({ isDefined_107(flexLine.with(|__s| __s.layout.remainingFreeSpace)) })
            && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_f32)
        {
            flexGrowFactor = ({ facebook_yoga_NodeImpl::resolveFlexGrow(&currentLineChild) });
            if (!(flexGrowFactor.is_nan())) && (flexGrowFactor != 0_f32) {
                updatedMainSize = ({
                    boundAxisWithAutoMin_460(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        ({ childFlexBasis } + {
                            ({
                                ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                                    flexLine.with(|__s| __s.layout.totalFlexGrowFactors)
                                })
                            } * { flexGrowFactor })
                        }),
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
            }
        }
        deltaFreeSpace += (updatedMainSize - childFlexBasis);
        let mut marginMain: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                mainAxis,
                availableInnerWidth,
            )
        });
        let mut marginCross: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                crossAxis,
                availableInnerWidth,
            )
        });
        let childCrossSize: Value<f32> =
            Rc::new(RefCell::new(YGUndefined_23.with(|rc| *rc.borrow())));
        let childMainSize: Value<f32> = Rc::new(RefCell::new((updatedMainSize + marginMain)));
        let childCrossSizingMode: Value<facebook_yoga_SizingMode> =
            <Value<facebook_yoga_SizingMode>>::default();
        let childMainSizingMode: Value<facebook_yoga_SizingMode> =
            Rc::new(RefCell::new(facebook_yoga_SizingMode_StretchFit));
        let childStyle: Ptr<facebook_yoga_Style> =
            ({ facebook_yoga_NodeImpl::style_24(&currentLineChild) });
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                ))
                .as_pointer(),
            )
        }) {
            (*childCrossSize.borrow_mut()) = if isMainAxisRow {
                ({ ((*childMainSize.borrow()) - marginMain) } / {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                            ))
                            .as_pointer(),
                        )
                    })
                })
            } else {
                ({ ((*childMainSize.borrow()) - marginMain) } * {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                            ))
                            .as_pointer(),
                        )
                    })
                })
            };
            (*childCrossSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
            (*childCrossSize.borrow_mut()) += marginCross;
        } else if ((((((!(availableInnerCrossDim.is_nan()))
            && (!({
                facebook_yoga_NodeImpl::hasDefiniteLength(
                    &currentLineChild,
                    ({ dimension_136(crossAxis) }),
                    availableInnerCrossDim,
                )
            })))
            && (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit))
            && (!((isNodeFlexWrap) && (mainAxisOverflows))))
            && (({ resolveChildAlignment_413((node).clone(), (currentLineChild).clone()) })
                == facebook_yoga_Align_Stretch))
            && (!({
                facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                    &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                    crossAxis,
                    direction,
                )
            })))
            && (!({
                facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                    &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                    crossAxis,
                    direction,
                )
            }))
        {
            (*childCrossSize.borrow_mut()) = availableInnerCrossDim;
            (*childCrossSizingMode.borrow_mut()) = facebook_yoga_SizingMode_StretchFit;
        } else if !({
            facebook_yoga_NodeImpl::hasDefiniteLength(
                &currentLineChild,
                ({ dimension_136(crossAxis) }),
                availableInnerCrossDim,
            )
        }) {
            (*childCrossSize.borrow_mut()) = availableInnerCrossDim;
            (*childCrossSizingMode.borrow_mut()) =
                if ({ isUndefined_105((*childCrossSize.borrow())) }) {
                    facebook_yoga_SizingMode_MaxContent
                } else {
                    facebook_yoga_SizingMode_FitContent
                };
        } else {
            (*childCrossSize.borrow_mut()) = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                facebook_yoga_NodeImpl::getResolvedDimension(
                                    &currentLineChild,
                                    direction,
                                    ({ dimension_136(crossAxis) }),
                                    availableInnerCrossDim,
                                    availableInnerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } + { marginCross });
            let mut isLoosePercentageMeasurement: bool = ({
                facebook_yoga_StyleSizeLengthImpl::isPercent(
                    &Rc::new(RefCell::new(
                        ({
                            facebook_yoga_NodeImpl::getProcessedDimension(
                                &currentLineChild,
                                ({ dimension_136(crossAxis) }),
                            )
                        }),
                    ))
                    .as_pointer(),
                )
            }) && (sizingModeCrossDim
                != facebook_yoga_SizingMode_StretchFit);
            (*childCrossSizingMode.borrow_mut()) =
                if ({ isUndefined_105((*childCrossSize.borrow())) })
                    || (isLoosePercentageMeasurement)
                {
                    facebook_yoga_SizingMode_MaxContent
                } else {
                    facebook_yoga_SizingMode_StretchFit
                };
        }
        ({
            constrainMaxSizeForMode_449(
                (currentLineChild).clone(),
                direction,
                mainAxis,
                availableInnerMainDim,
                availableInnerWidth,
                (childMainSizingMode.as_pointer()),
                (childMainSize.as_pointer()),
            )
        });
        ({
            constrainMaxSizeForMode_449(
                (currentLineChild).clone(),
                direction,
                crossAxis,
                availableInnerCrossDim,
                availableInnerWidth,
                (childCrossSizingMode.as_pointer()),
                (childCrossSize.as_pointer()),
            )
        });
        let mut requiresStretchLayout: bool = (((!({
            facebook_yoga_NodeImpl::hasDefiniteLength(
                &currentLineChild,
                ({ dimension_136(crossAxis) }),
                availableInnerCrossDim,
            )
        })) && (({
            resolveChildAlignment_413((node).clone(), (currentLineChild).clone())
        }) == facebook_yoga_Align_Stretch))
            && (!({
                facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                    &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                    crossAxis,
                    direction,
                )
            })))
            && (!({
                facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                    &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                    crossAxis,
                    direction,
                )
            }));
        let mut childWidth: f32 = if isMainAxisRow {
            (*childMainSize.borrow())
        } else {
            (*childCrossSize.borrow())
        };
        let mut childHeight: f32 = if !(isMainAxisRow) {
            (*childMainSize.borrow())
        } else {
            (*childCrossSize.borrow())
        };
        let mut childWidthSizingMode: facebook_yoga_SizingMode = if isMainAxisRow {
            (*childMainSizingMode.borrow())
        } else {
            (*childCrossSizingMode.borrow())
        };
        let mut childHeightSizingMode: facebook_yoga_SizingMode = if !(isMainAxisRow) {
            (*childMainSizingMode.borrow())
        } else {
            (*childCrossSizingMode.borrow())
        };
        let mut isLayoutPass: bool = (performLayout) && (!(requiresStretchLayout));
        ({
            let _node: Ptr<facebook_yoga_Node> = (currentLineChild).clone();
            let _availableWidth: f32 = childWidth;
            let _availableHeight: f32 = childHeight;
            let _ownerDirection: facebook_yoga_Direction = ({
                facebook_yoga_LayoutResultsImpl::direction(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                )
            });
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: f32 = availableInnerWidth;
            let _ownerHeight: f32 = availableInnerHeight;
            let _performLayout: bool = isLayoutPass;
            let _reason: facebook_yoga_LayoutPassReason = if isLayoutPass {
                facebook_yoga_LayoutPassReason_kFlexLayout
            } else {
                facebook_yoga_LayoutPassReason_kFlexMeasure
            };
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutInternal_430(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                _performLayout,
                _reason,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        ({
            let _hadOverflow: bool = ({
                facebook_yoga_LayoutResultsImpl::hadOverflow(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                )
            }) || ({
                facebook_yoga_LayoutResultsImpl::hadOverflow(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild) }),
                )
            });
            facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, _hadOverflow)
        });
    }
    return deltaFreeSpace;
}
pub fn distributeFreeSpaceFirstPass_462(
    mut node: Ptr<facebook_yoga_Node>,
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut ownerWidth: f32,
    mut mainAxisOwnerSize: f32,
    mut availableInnerMainDim: f32,
    mut availableInnerWidth: f32,
) {
    let mut flexShrinkScaledFactor: f32 = 0_f32;
    let mut flexGrowFactor: f32 = 0_f32;
    let mut baseMainSize: f32 = 0_f32;
    let mut boundMainSize: f32 = 0_f32;
    let mut deltaFreeSpace: f32 = 0_f32;
    let mut useRunningTotals: bool = ({
        facebook_yoga_NodeImpl::hasErrata(
            &node,
            facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
        )
    });
    let mut originalTotalFlexGrowFactors: f32 =
        flexLine.with(|__s| __s.layout.totalFlexGrowFactors);
    let mut originalTotalFlexShrinkScaledFactors: f32 =
        flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors);
    'loop_: for currentLineChild in IterRange::new(
        field_ref!(flexLine, itemsInFlow).decay(),
        field_ref!(flexLine, itemsInFlow).decay().to_end(),
    ) {
        let mut currentLineChild: Ptr<facebook_yoga_Node> = (currentLineChild.read());
        let mut childFlexBasis: f32 = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _node: Ptr<facebook_yoga_Node> = (currentLineChild).clone();
                        let _value: facebook_yoga_FloatOptional =
                            ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild) })
                                .with(|__s| __s.computedFlexBasis.clone());
                        boundAxisWithinMinAndMax_418(
                            _node,
                            direction,
                            mainAxis,
                            _value,
                            mainAxisOwnerSize,
                            ownerWidth,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
        if (flexLine.with(|__s| __s.layout.remainingFreeSpace) < 0_f32) {
            flexShrinkScaledFactor = ({
                -({ facebook_yoga_NodeImpl::resolveFlexShrink(&currentLineChild) })
            } * { childFlexBasis });
            if ({ isDefined_107(flexShrinkScaledFactor) }) && (flexShrinkScaledFactor != 0_f32) {
                baseMainSize = ({ childFlexBasis } + {
                    ({
                        ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                            (if useRunningTotals {
                                flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors)
                            } else {
                                originalTotalFlexShrinkScaledFactors
                            })
                        })
                    } * { flexShrinkScaledFactor })
                });
                boundMainSize = ({
                    boundAxisWithAutoMin_460(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if (({ isDefined_107(baseMainSize) }) && ({ isDefined_107(boundMainSize) }))
                    && (baseMainSize != boundMainSize)
                {
                    deltaFreeSpace += (boundMainSize - childFlexBasis);
                    {
                        let __rhs = ({
                            -({ facebook_yoga_NodeImpl::resolveFlexShrink(&currentLineChild) })
                        } * {
                            ({
                                facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!(
                                    ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild,) }),
                                    computedFlexBasis
                                ))
                            })
                        });
                        field!(field!(flexLine, layout), totalFlexShrinkScaledFactors)
                            .with_mut(|__v| *__v = *__v - __rhs)
                    };
                }
            }
        } else if ({ isDefined_107(flexLine.with(|__s| __s.layout.remainingFreeSpace)) })
            && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_f32)
        {
            flexGrowFactor = ({ facebook_yoga_NodeImpl::resolveFlexGrow(&currentLineChild) });
            if ({ isDefined_107(flexGrowFactor) }) && (flexGrowFactor != 0_f32) {
                baseMainSize = ({ childFlexBasis } + {
                    ({
                        ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                            (if useRunningTotals {
                                flexLine.with(|__s| __s.layout.totalFlexGrowFactors)
                            } else {
                                originalTotalFlexGrowFactors
                            })
                        })
                    } * { flexGrowFactor })
                });
                boundMainSize = ({
                    boundAxis_419(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if (({ isDefined_107(baseMainSize) }) && ({ isDefined_107(boundMainSize) }))
                    && (baseMainSize != boundMainSize)
                {
                    deltaFreeSpace += (boundMainSize - childFlexBasis);
                    {
                        let __rhs = flexGrowFactor;
                        field!(field!(flexLine, layout), totalFlexGrowFactors)
                            .with_mut(|__v| *__v = *__v - __rhs)
                    };
                }
            }
        }
    }
    {
        let __rhs = deltaFreeSpace;
        field!(field!(flexLine, layout), remainingFreeSpace).with_mut(|__v| *__v = *__v - __rhs)
    };
}
pub fn resolveFlexibleLength_463(
    mut node: Ptr<facebook_yoga_Node>,
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut ownerWidth: f32,
    mut mainAxisOwnerSize: f32,
    mut availableInnerMainDim: f32,
    mut availableInnerCrossDim: f32,
    mut availableInnerWidth: f32,
    mut availableInnerHeight: f32,
    mut mainAxisOverflows: bool,
    mut sizingModeCrossDim: facebook_yoga_SizingMode,
    mut performLayout: bool,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) {
    let mut originalFreeSpace: f32 = flexLine.with(|__s| __s.layout.remainingFreeSpace);
    if !({
        facebook_yoga_NodeImpl::hasErrata(&node, facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto)
    }) {
        'loop_: for currentLineChild in IterRange::new(
            field_ref!(flexLine, itemsInFlow).decay(),
            field_ref!(flexLine, itemsInFlow).decay().to_end(),
        ) {
            let mut currentLineChild: Ptr<facebook_yoga_Node> = (currentLineChild.read());
            let __rhs = ({
                computeAutoMinMainSize_459(
                    (currentLineChild).clone(),
                    mainAxis,
                    direction,
                    mainAxisOwnerSize,
                    availableInnerWidth,
                    availableInnerHeight,
                    generationCount,
                )
            });
            field!(
                ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild,) }),
                computedAutoMinMainSize
            )
            .write(__rhs);
        }
    } else {
        'loop_: for currentLineChild in IterRange::new(
            field_ref!(flexLine, itemsInFlow).decay(),
            field_ref!(flexLine, itemsInFlow).decay().to_end(),
        ) {
            let mut currentLineChild: Ptr<facebook_yoga_Node> = (currentLineChild.read());
            let __rhs = <facebook_yoga_FloatOptional>::default();
            field!(
                ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild,) }),
                computedAutoMinMainSize
            )
            .write(__rhs);
        }
    }
    ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _flexLine: Ptr<facebook_yoga_FlexLine> = (flexLine).clone();
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _ownerWidth: f32 = ownerWidth;
        let _mainAxisOwnerSize: f32 = mainAxisOwnerSize;
        let _availableInnerMainDim: f32 = availableInnerMainDim;
        let _availableInnerWidth: f32 = availableInnerWidth;
        distributeFreeSpaceFirstPass_462(
            _node,
            _flexLine,
            _direction,
            _mainAxis,
            _ownerWidth,
            _mainAxisOwnerSize,
            _availableInnerMainDim,
            _availableInnerWidth,
        )
    });
    let mut distributedFreeSpace: f32 = ({
        let _flexLine: Ptr<facebook_yoga_FlexLine> = (flexLine).clone();
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
        let _direction: facebook_yoga_Direction = direction;
        let _ownerWidth: f32 = ownerWidth;
        let _mainAxisOwnerSize: f32 = mainAxisOwnerSize;
        let _availableInnerMainDim: f32 = availableInnerMainDim;
        let _availableInnerCrossDim: f32 = availableInnerCrossDim;
        let _availableInnerWidth: f32 = availableInnerWidth;
        let _availableInnerHeight: f32 = availableInnerHeight;
        let _mainAxisOverflows: bool = mainAxisOverflows;
        let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
        let _performLayout: bool = performLayout;
        let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
        let _depth: u32 = depth;
        let _generationCount: u32 = generationCount;
        distributeFreeSpaceSecondPass_461(
            _flexLine,
            _node,
            _mainAxis,
            _crossAxis,
            _direction,
            _ownerWidth,
            _mainAxisOwnerSize,
            _availableInnerMainDim,
            _availableInnerCrossDim,
            _availableInnerWidth,
            _availableInnerHeight,
            _mainAxisOverflows,
            _sizingModeCrossDim,
            _performLayout,
            _layoutMarkerData,
            _depth,
            _generationCount,
        )
    });
    field!(field!(flexLine, layout), remainingFreeSpace)
        .write((originalFreeSpace - distributedFreeSpace));
}
pub fn justifyMainAxis_464(
    mut node: Ptr<facebook_yoga_Node>,
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut sizingModeMainDim: facebook_yoga_SizingMode,
    mut sizingModeCrossDim: facebook_yoga_SizingMode,
    mut mainAxisOwnerSize: f32,
    mut ownerWidth: f32,
    mut availableInnerMainDim: f32,
    mut availableInnerCrossDim: f32,
    mut availableInnerWidth: f32,
    mut performLayout: bool,
) {
    let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&node) });
    let mut leadingPaddingAndBorderMain: f32 = ({
        facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let mut trailingPaddingAndBorderMain: f32 = ({
        facebook_yoga_StyleImpl::computeFlexEndPaddingAndBorder(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let mut gap: f32 = ({
        facebook_yoga_StyleImpl::computeGapForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            availableInnerMainDim,
        )
    });
    if (sizingModeMainDim == facebook_yoga_SizingMode_FitContent)
        && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_f32)
    {
        if ({
            facebook_yoga_StyleSizeLengthImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _axis: facebook_yoga_Dimension = ({ dimension_136(mainAxis) });
                        facebook_yoga_StyleImpl::minDimension(&style, _axis)
                    }),
                ))
                .as_pointer(),
            )
        }) && ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _direction: facebook_yoga_Direction = direction;
                        let _axis: facebook_yoga_Dimension = ({ dimension_136(mainAxis) });
                        let _referenceLength: f32 = mainAxisOwnerSize;
                        let _ownerWidth: f32 = ownerWidth;
                        facebook_yoga_StyleImpl::resolvedMinDimension(
                            &style,
                            _direction,
                            _axis,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                ))
                .as_pointer(),
            )
        }) {
            let mut minAvailableMainDim: f32 = ({
                ({
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({
                                    let _direction: facebook_yoga_Direction = direction;
                                    let _axis: facebook_yoga_Dimension =
                                        ({ dimension_136(mainAxis) });
                                    let _referenceLength: f32 = mainAxisOwnerSize;
                                    let _ownerWidth: f32 = ownerWidth;
                                    facebook_yoga_StyleImpl::resolvedMinDimension(
                                        &style,
                                        _direction,
                                        _axis,
                                        _referenceLength,
                                        _ownerWidth,
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    })
                } - { leadingPaddingAndBorderMain })
            } - { trailingPaddingAndBorderMain });
            let mut occupiedSpaceByChildNodes: f32 = ({ availableInnerMainDim } - {
                flexLine.with(|__s| __s.layout.remainingFreeSpace)
            });
            field!(field!(flexLine, layout), remainingFreeSpace).write(
                ({ maxOrDefined_110(0_f32, (minAvailableMainDim - occupiedSpaceByChildNodes)) }),
            );
        } else {
            field!(field!(flexLine, layout), remainingFreeSpace).write(0_f32);
        }
    }
    let mut leadingMainDim: f32 = 0_f32;
    let mut betweenMainDim: f32 = gap;
    let mut justifyContent: facebook_yoga_Justify = if (flexLine
        .with(|__s| __s.layout.remainingFreeSpace)
        >= 0_f32)
    {
        ({
            facebook_yoga_StyleImpl::justifyContent(&({ facebook_yoga_NodeImpl::style_24(&node) }))
        })
    } else {
        ({
            fallbackAlignment_416(
                ({
                    facebook_yoga_StyleImpl::justifyContent(
                        &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    )
                }),
            )
        })
    };
    if (flexLine.with(|__s| __s.numberOfAutoMargins) == 0_usize) {
        'switch: {
            match { justifyContent } {
                __v if __v == facebook_yoga_Justify_Start
                    || __v == facebook_yoga_Justify_End
                    || __v == facebook_yoga_Justify_Auto =>
                {
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_Stretch => {
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_Center => {
                    leadingMainDim = (flexLine.with(|__s| __s.layout.remainingFreeSpace) / 2_f32);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_FlexEnd => {
                    leadingMainDim = flexLine.with(|__s| __s.layout.remainingFreeSpace);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceBetween => {
                    if ((*flexLine.upgrade().deref()).itemsInFlow.len() > 1_usize) {
                        betweenMainDim += ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) }
                            / {
                                ((((*flexLine.upgrade().deref()).itemsInFlow.len())
                                    .wrapping_sub(1_usize)) as f32)
                            });
                    }
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceEvenly => {
                    leadingMainDim = ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                        ((((*flexLine.upgrade().deref()).itemsInFlow.len()).wrapping_add(1_usize))
                            as f32)
                    });
                    betweenMainDim += leadingMainDim;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceAround => {
                    leadingMainDim =
                        ({ (0.5_f32 * flexLine.with(|__s| __s.layout.remainingFreeSpace)) } / {
                            ((*flexLine.upgrade().deref()).itemsInFlow.len() as f32)
                        });
                    betweenMainDim += (leadingMainDim * 2_f32);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_FlexStart => {
                    break 'switch;
                }
                _ => {}
            }
        };
    }
    field!(field!(flexLine, layout), mainDim).write((leadingPaddingAndBorderMain + leadingMainDim));
    field!(field!(flexLine, layout), crossDim).write(0_f32);
    let mut maxAscentForCurrentLine: f32 = 0_f32;
    let mut maxDescentForCurrentLine: f32 = 0_f32;
    let mut isNodeBaselineLayout: bool = ({ isBaselineLayout_434((node).clone()) });
    'loop_: for child in IterRange::new(
        field_ref!(flexLine, itemsInFlow).decay(),
        field_ref!(flexLine, itemsInFlow).decay().to_end(),
    ) {
        let mut child: Ptr<facebook_yoga_Node> = (child.read());
        let childLayout: Ptr<facebook_yoga_LayoutResults> =
            ({ facebook_yoga_NodeImpl::getLayout_26(&child) });
        if ({
            facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                mainAxis,
                direction,
            )
        }) && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_f32)
        {
            {
                let __rhs = {
                    ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                        (flexLine.with(|__s| __s.numberOfAutoMargins) as f32)
                    })
                };
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
        }
        if performLayout {
            ({
                let _position: f32 = ({
                    ({
                        let _physicalEdge: facebook_yoga_PhysicalEdge =
                            ({ flexStartEdge_132(mainAxis) });
                        facebook_yoga_LayoutResultsImpl::position(&childLayout, _physicalEdge)
                    })
                } + { flexLine.with(|__s| __s.layout.mainDim) });
                let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_132(mainAxis) });
                facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
            });
        }
        if ({ (child).clone() } != { (field_ref!(flexLine, itemsInFlow).decay().to_last().read()) })
        {
            {
                let __rhs = betweenMainDim;
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
        }
        if ({
            facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                mainAxis,
                direction,
            )
        }) && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_f32)
        {
            {
                let __rhs = {
                    ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                        (flexLine.with(|__s| __s.numberOfAutoMargins) as f32)
                    })
                };
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
        }
        let mut canSkipFlex: bool =
            (!(performLayout)) && (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit);
        if canSkipFlex {
            {
                let __rhs = ({
                    ({
                        facebook_yoga_StyleImpl::computeMarginForAxis(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            mainAxis,
                            availableInnerWidth,
                        )
                    })
                } + {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({
                                    boundAxisWithinMinAndMax_418(
                                        (child).clone(),
                                        direction,
                                        mainAxis,
                                        childLayout.with(|__s| __s.computedFlexBasis.clone()),
                                        mainAxisOwnerSize,
                                        ownerWidth,
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    })
                });
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
            field!(field!(flexLine, layout), crossDim).write(availableInnerCrossDim);
        } else {
            {
                let __rhs = ({
                    facebook_yoga_NodeImpl::dimensionWithMargin(
                        &child,
                        mainAxis,
                        availableInnerWidth,
                    )
                });
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
            if isNodeBaselineLayout {
                let mut ascent: f32 = ({ ({ calculateBaseline_433((child).clone()) }) } + {
                    ({
                        facebook_yoga_StyleImpl::computeFlexStartMargin(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            facebook_yoga_FlexDirection_Column,
                            direction,
                            availableInnerWidth,
                        )
                    })
                });
                let mut descent: f32 = ({
                    ({
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                facebook_yoga_Dimension_Height,
                            )
                        })
                    } + {
                        ({
                            facebook_yoga_StyleImpl::computeMarginForAxis(
                                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                facebook_yoga_FlexDirection_Column,
                                availableInnerWidth,
                            )
                        })
                    })
                } - { ascent });
                let __rhs = ({ maxOrDefined_110(maxAscentForCurrentLine, ascent) });
                maxAscentForCurrentLine = __rhs;
                let __rhs = ({ maxOrDefined_110(maxDescentForCurrentLine, descent) });
                maxDescentForCurrentLine = __rhs;
            } else {
                let __rhs = ({
                    maxOrDefined_110(
                        flexLine.with(|__s| __s.layout.crossDim),
                        ({
                            facebook_yoga_NodeImpl::dimensionWithMargin(
                                &child,
                                crossAxis,
                                availableInnerWidth,
                            )
                        }),
                    )
                });
                field!(field!(flexLine, layout), crossDim).write(__rhs);
            }
        }
    }
    {
        let __rhs = trailingPaddingAndBorderMain;
        field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
    };
    if isNodeBaselineLayout {
        field!(field!(flexLine, layout), crossDim)
            .write((maxAscentForCurrentLine + maxDescentForCurrentLine));
    }
}
pub fn calculateLayoutImpl_465(
    mut node: Ptr<facebook_yoga_Node>,
    mut availableWidth: f32,
    mut availableHeight: f32,
    mut ownerDirection: facebook_yoga_Direction,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut performLayout: bool,
    mut reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) {
    ({
        assertFatalWithNode_213(
            (node).clone(),
            if ({ isUndefined_105(availableWidth) }) {
                (widthSizingMode == facebook_yoga_SizingMode_MaxContent)
            } else {
                true
            },
            Ptr::<i8>::from_string_literal(
                b"availableWidth is indefinite so widthSizingMode must be SizingMode::MaxContent",
            ),
        )
    });
    ({
        assertFatalWithNode_213(
            (node).clone(),
            if ({ isUndefined_105(availableHeight) }) {
                (heightSizingMode == facebook_yoga_SizingMode_MaxContent)
            } else {
                true
            },
            Ptr::<i8>::from_string_literal(
                b"availableHeight is indefinite so heightSizingMode must be SizingMode::MaxContent",
            ),
        )
    });
    {
        if performLayout {
            (field_ptr!(layoutMarkerData, layouts))
        } else {
            (field_ptr!(layoutMarkerData, measures))
        }
        .with_mut(|__v| *__v = *__v + 1)
    };
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    ({ facebook_yoga_NodeImpl::setLayoutDirection(&node, direction) });
    let mut fixFlexBasisFitContent: bool = ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    if (fixFlexBasisFitContent) && (performLayout) {
        ({ facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, false) });
    }
    let mut flexRowDirection: facebook_yoga_FlexDirection =
        ({ resolveDirection_130(facebook_yoga_FlexDirection_Row, direction) });
    let mut flexColumnDirection: facebook_yoga_FlexDirection =
        ({ resolveDirection_130(facebook_yoga_FlexDirection_Column, direction) });
    let mut startEdge: facebook_yoga_PhysicalEdge = if (direction == facebook_yoga_Direction_LTR) {
        facebook_yoga_PhysicalEdge_Left
    } else {
        facebook_yoga_PhysicalEdge_Right
    };
    let mut endEdge: facebook_yoga_PhysicalEdge = if (direction == facebook_yoga_Direction_LTR) {
        facebook_yoga_PhysicalEdge_Right
    } else {
        facebook_yoga_PhysicalEdge_Left
    };
    let mut marginRowLeading: f32 = ({
        facebook_yoga_StyleImpl::computeInlineStartMargin(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    ({ facebook_yoga_NodeImpl::setLayoutMargin(&node, marginRowLeading, startEdge) });
    let mut marginRowTrailing: f32 = ({
        facebook_yoga_StyleImpl::computeInlineEndMargin(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    ({ facebook_yoga_NodeImpl::setLayoutMargin(&node, marginRowTrailing, endEdge) });
    let mut marginColumnLeading: f32 = ({
        facebook_yoga_StyleImpl::computeInlineStartMargin(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            flexColumnDirection,
            direction,
            ownerWidth,
        )
    });
    ({
        facebook_yoga_NodeImpl::setLayoutMargin(
            &node,
            marginColumnLeading,
            facebook_yoga_PhysicalEdge_Top,
        )
    });
    let mut marginColumnTrailing: f32 = ({
        facebook_yoga_StyleImpl::computeInlineEndMargin(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            flexColumnDirection,
            direction,
            ownerWidth,
        )
    });
    ({
        facebook_yoga_NodeImpl::setLayoutMargin(
            &node,
            marginColumnTrailing,
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
    let mut marginAxisRow: f32 = (marginRowLeading + marginRowTrailing);
    let mut marginAxisColumn: f32 = (marginColumnLeading + marginColumnTrailing);
    ({
        let _border: f32 = ({
            facebook_yoga_StyleImpl::computeInlineStartBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, startEdge)
    });
    ({
        let _border: f32 = ({
            facebook_yoga_StyleImpl::computeInlineEndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, endEdge)
    });
    ({
        let _border: f32 = ({
            facebook_yoga_StyleImpl::computeInlineStartBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, facebook_yoga_PhysicalEdge_Top)
    });
    ({
        let _border: f32 = ({
            facebook_yoga_StyleImpl::computeInlineEndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, facebook_yoga_PhysicalEdge_Bottom)
    });
    ({
        let _padding: f32 = ({
            facebook_yoga_StyleImpl::computeInlineStartPadding(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexRowDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_NodeImpl::setLayoutPadding(&node, _padding, startEdge)
    });
    ({
        let _padding: f32 = ({
            facebook_yoga_StyleImpl::computeInlineEndPadding(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexRowDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_NodeImpl::setLayoutPadding(&node, _padding, endEdge)
    });
    ({
        let _padding: f32 = ({
            facebook_yoga_StyleImpl::computeInlineStartPadding(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexColumnDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_NodeImpl::setLayoutPadding(&node, _padding, facebook_yoga_PhysicalEdge_Top)
    });
    ({
        let _padding: f32 = ({
            facebook_yoga_StyleImpl::computeInlineEndPadding(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexColumnDirection,
                direction,
                ownerWidth,
            )
        });
        facebook_yoga_NodeImpl::setLayoutPadding(&node, _padding, facebook_yoga_PhysicalEdge_Bottom)
    });
    if ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) }) {
        ({
            let _node: Ptr<facebook_yoga_Node> = (node).clone();
            let _direction: facebook_yoga_Direction = direction;
            let _availableWidth: f32 = (availableWidth - marginAxisRow);
            let _availableHeight: f32 = (availableHeight - marginAxisColumn);
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: f32 = ownerWidth;
            let _ownerHeight: f32 = ownerHeight;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _reason: facebook_yoga_LayoutPassReason = reason;
            measureNodeWithMeasureFunc_451(
                _node,
                _direction,
                _availableWidth,
                _availableHeight,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                _layoutMarkerData,
                _reason,
            )
        });
        ({ cleanupContentsNodesRecursively_432((node).clone(), performLayout) });
        return;
    }
    let mut childCount: usize = ({ facebook_yoga_NodeImpl::getLayoutChildCount(&node) });
    if (childCount == 0_usize) {
        ({
            measureNodeWithoutChildren_452(
                (node).clone(),
                direction,
                (availableWidth - marginAxisRow),
                (availableHeight - marginAxisColumn),
                widthSizingMode,
                heightSizingMode,
                ownerWidth,
                ownerHeight,
            )
        });
        ({ cleanupContentsNodesRecursively_432((node).clone(), performLayout) });
        return;
    }
    if (!(performLayout))
        && ({
            measureNodeWithFixedSize_454(
                (node).clone(),
                direction,
                (availableWidth - marginAxisRow),
                (availableHeight - marginAxisColumn),
                widthSizingMode,
                heightSizingMode,
                ownerWidth,
                ownerHeight,
            )
        })
    {
        ({ cleanupContentsNodesRecursively_432((node).clone(), false) });
        return;
    }
    ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&node) });
    if (!(fixFlexBasisFitContent)) || (!(performLayout)) {
        ({ facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, false) });
    }
    ({ cleanupContentsNodesRecursively_432((node).clone(), performLayout) });
    let mut mainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_130(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            }),
            direction,
        )
    });
    let mut crossAxis: facebook_yoga_FlexDirection =
        ({ resolveCrossDirection_131(mainAxis, direction) });
    let mut isMainAxisRow: bool = ({ isRow_128(mainAxis) });
    let mut isNodeFlexWrap: bool =
        (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            != facebook_yoga_Wrap_NoWrap);
    let mut mainAxisOwnerSize: f32 = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let mut crossAxisOwnerSize: f32 = if isMainAxisRow {
        ownerHeight
    } else {
        ownerWidth
    };
    let mut paddingAndBorderAxisMain: f32 =
        ({ paddingAndBorderForAxis_417((node).clone(), mainAxis, direction, ownerWidth) });
    let mut paddingAndBorderAxisCross: f32 =
        ({ paddingAndBorderForAxis_417((node).clone(), crossAxis, direction, ownerWidth) });
    let mut leadingPaddingAndBorderCross: f32 = ({
        facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            crossAxis,
            direction,
            ownerWidth,
        )
    });
    let mut sizingModeMainDim: facebook_yoga_SizingMode = if isMainAxisRow {
        widthSizingMode
    } else {
        heightSizingMode
    };
    let mut sizingModeCrossDim: facebook_yoga_SizingMode = if isMainAxisRow {
        heightSizingMode
    } else {
        widthSizingMode
    };
    let mut paddingAndBorderAxisRow: f32 = if isMainAxisRow {
        paddingAndBorderAxisMain
    } else {
        paddingAndBorderAxisCross
    };
    let mut paddingAndBorderAxisColumn: f32 = if isMainAxisRow {
        paddingAndBorderAxisCross
    } else {
        paddingAndBorderAxisMain
    };
    let mut availableInnerWidth: f32 = ({
        let _ownerDim: f32 = ownerWidth;
        let _ownerWidth: f32 = ownerWidth;
        calculateAvailableInnerDimension_456(
            (node).clone(),
            direction,
            facebook_yoga_Dimension_Width,
            (availableWidth - marginAxisRow),
            paddingAndBorderAxisRow,
            _ownerDim,
            _ownerWidth,
        )
    });
    let mut availableInnerHeight: f32 = ({
        calculateAvailableInnerDimension_456(
            (node).clone(),
            direction,
            facebook_yoga_Dimension_Height,
            (availableHeight - marginAxisColumn),
            paddingAndBorderAxisColumn,
            ownerHeight,
            ownerWidth,
        )
    });
    let mut availableInnerMainDim: f32 = if isMainAxisRow {
        availableInnerWidth
    } else {
        availableInnerHeight
    };
    let mut availableInnerCrossDim: f32 = if isMainAxisRow {
        availableInnerHeight
    } else {
        availableInnerWidth
    };
    let mut totalMainDim: f32 = 0_f32;
    totalMainDim += ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _availableInnerWidth: f32 = availableInnerWidth;
        let _availableInnerHeight: f32 = availableInnerHeight;
        let _ownerWidth: f32 = availableInnerWidth;
        let _ownerHeight: f32 = availableInnerHeight;
        let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
        let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _performLayout: bool = performLayout;
        let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
        let _depth: u32 = depth;
        let _generationCount: u32 = generationCount;
        computeFlexBasisForChildren_457(
            _node,
            _availableInnerWidth,
            _availableInnerHeight,
            _ownerWidth,
            _ownerHeight,
            _widthSizingMode,
            _heightSizingMode,
            _direction,
            _mainAxis,
            _performLayout,
            _layoutMarkerData,
            _depth,
            _generationCount,
        )
    });
    if (childCount > 1_usize) {
        totalMainDim += ({
            ({
                facebook_yoga_StyleImpl::computeGapForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    mainAxis,
                    availableInnerMainDim,
                )
            })
        } * { (((childCount).wrapping_sub(1_usize)) as f32) });
    }
    let mut mainAxisOverflows: bool = (sizingModeMainDim != facebook_yoga_SizingMode_MaxContent)
        && (totalMainDim > availableInnerMainDim);
    if ((isNodeFlexWrap) && (mainAxisOverflows))
        && (sizingModeMainDim == facebook_yoga_SizingMode_FitContent)
    {
        sizingModeMainDim = facebook_yoga_SizingMode_StretchFit;
    }
    let startOfLineIterator: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_NodeImpl::getLayoutChildren(&node) }),
                    ))
                    .as_pointer(),
                )
            }),
        ));
    let mut lineCount: usize = 0_usize;
    let mut totalLineCrossDim: f32 = 0_f32;
    let mut crossAxisGap: f32 = ({
        facebook_yoga_StyleImpl::computeGapForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            crossAxis,
            availableInnerCrossDim,
        )
    });
    let mut maxLineMainDim: f32 = 0_f32;
    'loop_: while ({
        let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            startOfLineIterator.as_pointer();
        let _b: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(
                        &Rc::new(RefCell::new(
                            ({ facebook_yoga_NodeImpl::getLayoutChildren(&node) }),
                        ))
                        .as_pointer(),
                    )
                }),
            ));
        operator_ne_78(_a, _b.as_pointer())
    }) {
        let flexLine: Value<facebook_yoga_FlexLine> = Rc::new(RefCell::new(
            ({
                calculateFlexLine_466(
                    (node).clone(),
                    ownerDirection,
                    ownerWidth,
                    mainAxisOwnerSize,
                    availableInnerWidth,
                    availableInnerMainDim,
                    startOfLineIterator.as_pointer(),
                    lineCount,
                )
            }),
        ));
        let mut canSkipFlex: bool =
            (!(performLayout)) && (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit);
        let mut sizeBasedOnContent: bool = false;
        if (sizingModeMainDim != facebook_yoga_SizingMode_StretchFit) {
            let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&node) });
            let mut minInnerWidth: f32 = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: f32 = ownerWidth;
                                let _ownerWidth: f32 = ownerWidth;
                                facebook_yoga_StyleImpl::resolvedMinDimension(
                                    &style,
                                    _direction,
                                    facebook_yoga_Dimension_Width,
                                    _referenceLength,
                                    _ownerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } - { paddingAndBorderAxisRow });
            let mut maxInnerWidth: f32 = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: f32 = ownerWidth;
                                let _ownerWidth: f32 = ownerWidth;
                                facebook_yoga_StyleImpl::resolvedMaxDimension(
                                    &style,
                                    _direction,
                                    facebook_yoga_Dimension_Width,
                                    _referenceLength,
                                    _ownerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } - { paddingAndBorderAxisRow });
            let mut minInnerHeight: f32 = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: f32 = ownerHeight;
                                let _ownerWidth: f32 = ownerWidth;
                                facebook_yoga_StyleImpl::resolvedMinDimension(
                                    &style,
                                    _direction,
                                    facebook_yoga_Dimension_Height,
                                    _referenceLength,
                                    _ownerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } - { paddingAndBorderAxisColumn });
            let mut maxInnerHeight: f32 = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: f32 = ownerHeight;
                                let _ownerWidth: f32 = ownerWidth;
                                facebook_yoga_StyleImpl::resolvedMaxDimension(
                                    &style,
                                    _direction,
                                    facebook_yoga_Dimension_Height,
                                    _referenceLength,
                                    _ownerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } - { paddingAndBorderAxisColumn });
            let mut minInnerMainDim: f32 = if isMainAxisRow {
                minInnerWidth
            } else {
                minInnerHeight
            };
            let mut maxInnerMainDim: f32 = if isMainAxisRow {
                maxInnerWidth
            } else {
                maxInnerHeight
            };
            if ({ isDefined_107(minInnerMainDim) })
                && ({ (*flexLine.borrow()).sizeConsumed } < minInnerMainDim)
            {
                availableInnerMainDim = minInnerMainDim;
            } else if ({ isDefined_107(maxInnerMainDim) })
                && ({ (*flexLine.borrow()).sizeConsumed } > maxInnerMainDim)
            {
                availableInnerMainDim = maxInnerMainDim;
            } else {
                let mut useLegacyStretchBehaviour: bool = ({
                    facebook_yoga_NodeImpl::hasErrata(&node, facebook_yoga_Errata_StretchFlexBasis)
                });
                if (!(useLegacyStretchBehaviour))
                    && ((({ isDefined_107({ (*flexLine.borrow()).layout.totalFlexGrowFactors }) })
                        && ({ (*flexLine.borrow()).layout.totalFlexGrowFactors } == 0_f32))
                        || (({
                            isDefined_107(({ facebook_yoga_NodeImpl::resolveFlexGrow(&node) }))
                        }) && (({ facebook_yoga_NodeImpl::resolveFlexGrow(&node) }) == 0_f32)))
                {
                    availableInnerMainDim = { (*flexLine.borrow()).sizeConsumed };
                }
                sizeBasedOnContent = !(useLegacyStretchBehaviour);
            }
        }
        if (!(sizeBasedOnContent)) && ({ isDefined_107(availableInnerMainDim) }) {
            (*flexLine.borrow_mut()).layout.remainingFreeSpace =
                { (availableInnerMainDim - { (*flexLine.borrow()).sizeConsumed }) };
        } else if ({ (*flexLine.borrow()).sizeConsumed } < 0_f32) {
            (*flexLine.borrow_mut()).layout.remainingFreeSpace =
                { -{ (*flexLine.borrow()).sizeConsumed } };
        }
        if !(canSkipFlex) {
            ({
                let _node: Ptr<facebook_yoga_Node> = (node).clone();
                let _flexLine: Ptr<facebook_yoga_FlexLine> = flexLine.as_pointer();
                let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
                let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
                let _direction: facebook_yoga_Direction = direction;
                let _ownerWidth: f32 = ownerWidth;
                let _mainAxisOwnerSize: f32 = mainAxisOwnerSize;
                let _availableInnerMainDim: f32 = availableInnerMainDim;
                let _availableInnerCrossDim: f32 = availableInnerCrossDim;
                let _availableInnerWidth: f32 = availableInnerWidth;
                let _availableInnerHeight: f32 = availableInnerHeight;
                let _mainAxisOverflows: bool = mainAxisOverflows;
                let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
                let _performLayout: bool = performLayout;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _depth: u32 = depth;
                let _generationCount: u32 = generationCount;
                resolveFlexibleLength_463(
                    _node,
                    _flexLine,
                    _mainAxis,
                    _crossAxis,
                    _direction,
                    _ownerWidth,
                    _mainAxisOwnerSize,
                    _availableInnerMainDim,
                    _availableInnerCrossDim,
                    _availableInnerWidth,
                    _availableInnerHeight,
                    _mainAxisOverflows,
                    _sizingModeCrossDim,
                    _performLayout,
                    _layoutMarkerData,
                    _depth,
                    _generationCount,
                )
            });
        }
        ({
            let _hadOverflow: bool = ({
                facebook_yoga_LayoutResultsImpl::hadOverflow(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                )
            }) || ({ (*flexLine.borrow()).layout.remainingFreeSpace }
                < 0_f32);
            facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, _hadOverflow)
        });
        ({
            justifyMainAxis_464(
                (node).clone(),
                flexLine.as_pointer(),
                mainAxis,
                crossAxis,
                direction,
                sizingModeMainDim,
                sizingModeCrossDim,
                mainAxisOwnerSize,
                ownerWidth,
                availableInnerMainDim,
                availableInnerCrossDim,
                availableInnerWidth,
                performLayout,
            )
        });
        let mut containerCrossAxis: f32 = availableInnerCrossDim;
        if (sizingModeCrossDim == facebook_yoga_SizingMode_MaxContent)
            || (sizingModeCrossDim == facebook_yoga_SizingMode_FitContent)
        {
            containerCrossAxis = ({
                ({
                    boundAxis_419(
                        (node).clone(),
                        crossAxis,
                        direction,
                        ({ (*flexLine.borrow()).layout.crossDim } + paddingAndBorderAxisCross),
                        crossAxisOwnerSize,
                        ownerWidth,
                    )
                })
            } - { paddingAndBorderAxisCross });
        }
        if (!(isNodeFlexWrap)) && (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit) {
            (*flexLine.borrow_mut()).layout.crossDim = availableInnerCrossDim;
        }
        if !(isNodeFlexWrap) {
            let __rhs = ({
                ({
                    boundAxis_419(
                        (node).clone(),
                        crossAxis,
                        direction,
                        ({ (*flexLine.borrow()).layout.crossDim } + paddingAndBorderAxisCross),
                        crossAxisOwnerSize,
                        ownerWidth,
                    )
                })
            } - { paddingAndBorderAxisCross });
            (*flexLine.borrow_mut()).layout.crossDim = __rhs;
        }
        if performLayout {
            'loop_: for child in IterRange::new(
                field_ref!(flexLine.as_pointer(), itemsInFlow).decay(),
                field_ref!(flexLine.as_pointer(), itemsInFlow)
                    .decay()
                    .to_end(),
            ) {
                let mut child: Ptr<facebook_yoga_Node> = (child.read());
                let mut leadingCrossDim: f32 = leadingPaddingAndBorderCross;
                let mut alignItem: facebook_yoga_Align =
                    ({ resolveChildAlignment_413((node).clone(), (child).clone()) });
                if ((alignItem == facebook_yoga_Align_Stretch)
                    && (!({
                        facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            crossAxis,
                            direction,
                        )
                    })))
                    && (!({
                        facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            crossAxis,
                            direction,
                        )
                    }))
                {
                    if !({
                        facebook_yoga_NodeImpl::hasDefiniteLength(
                            &child,
                            ({ dimension_136(crossAxis) }),
                            availableInnerCrossDim,
                        )
                    }) {
                        let childMainSize: Value<f32> = Rc::new(RefCell::new(
                            ({
                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                    ({ dimension_136(mainAxis) }),
                                )
                            }),
                        ));
                        let childStyle: Ptr<facebook_yoga_Style> =
                            ({ facebook_yoga_NodeImpl::style_24(&child) });
                        let childCrossSize: Value<f32> = Rc::new(RefCell::new(
                            if ({
                                facebook_yoga_FloatOptionalImpl::isDefined(
                                    &Rc::new(RefCell::new(
                                        ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                                    ))
                                    .as_pointer(),
                                )
                            }) {
                                ({
                                    ({
                                        facebook_yoga_StyleImpl::computeMarginForAxis(
                                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                            crossAxis,
                                            availableInnerWidth,
                                        )
                                    })
                                } + {
                                    (if isMainAxisRow {
                                        ({ (*childMainSize.borrow()) } / {
                                            ({
                                                facebook_yoga_FloatOptionalImpl::unwrap(
                                                    &Rc::new(RefCell::new(
                                                        ({
                                                            facebook_yoga_StyleImpl::aspectRatio(
                                                                &childStyle,
                                                            )
                                                        }),
                                                    ))
                                                    .as_pointer(),
                                                )
                                            })
                                        })
                                    } else {
                                        ({ (*childMainSize.borrow()) } * {
                                            ({
                                                facebook_yoga_FloatOptionalImpl::unwrap(
                                                    &Rc::new(RefCell::new(
                                                        ({
                                                            facebook_yoga_StyleImpl::aspectRatio(
                                                                &childStyle,
                                                            )
                                                        }),
                                                    ))
                                                    .as_pointer(),
                                                )
                                            })
                                        })
                                    })
                                })
                            } else {
                                { (*flexLine.borrow()).layout.crossDim }
                            },
                        ));
                        (*childMainSize.borrow_mut()) += ({
                            facebook_yoga_StyleImpl::computeMarginForAxis(
                                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                mainAxis,
                                availableInnerWidth,
                            )
                        });
                        let childMainSizingMode: Value<facebook_yoga_SizingMode> =
                            Rc::new(RefCell::new(facebook_yoga_SizingMode_StretchFit));
                        let childCrossSizingMode: Value<facebook_yoga_SizingMode> =
                            Rc::new(RefCell::new(facebook_yoga_SizingMode_StretchFit));
                        ({
                            constrainMaxSizeForMode_449(
                                (child).clone(),
                                direction,
                                mainAxis,
                                availableInnerMainDim,
                                availableInnerWidth,
                                (childMainSizingMode.as_pointer()),
                                (childMainSize.as_pointer()),
                            )
                        });
                        ({
                            constrainMaxSizeForMode_449(
                                (child).clone(),
                                direction,
                                crossAxis,
                                availableInnerCrossDim,
                                availableInnerWidth,
                                (childCrossSizingMode.as_pointer()),
                                (childCrossSize.as_pointer()),
                            )
                        });
                        let mut childWidth: f32 = if isMainAxisRow {
                            (*childMainSize.borrow())
                        } else {
                            (*childCrossSize.borrow())
                        };
                        let mut childHeight: f32 = if !(isMainAxisRow) {
                            (*childMainSize.borrow())
                        } else {
                            (*childCrossSize.borrow())
                        };
                        let mut alignContent: facebook_yoga_Align = ({
                            facebook_yoga_StyleImpl::alignContent(
                                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                            )
                        });
                        let mut crossAxisDoesNotGrow: bool =
                            (alignContent != facebook_yoga_Align_Stretch) && (isNodeFlexWrap);
                        let mut childWidthSizingMode: facebook_yoga_SizingMode =
                            if ({ isUndefined_105(childWidth) })
                                || ((!(isMainAxisRow)) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        let mut childHeightSizingMode: facebook_yoga_SizingMode =
                            if ({ isUndefined_105(childHeight) })
                                || ((isMainAxisRow) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        ({
                            let _node: Ptr<facebook_yoga_Node> = (child).clone();
                            let _availableWidth: f32 = childWidth;
                            let _availableHeight: f32 = childHeight;
                            let _ownerDirection: facebook_yoga_Direction = direction;
                            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
                            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
                            let _ownerWidth: f32 = availableInnerWidth;
                            let _ownerHeight: f32 = availableInnerHeight;
                            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> =
                                (layoutMarkerData).clone();
                            let _depth: u32 = depth;
                            let _generationCount: u32 = generationCount;
                            calculateLayoutInternal_430(
                                _node,
                                _availableWidth,
                                _availableHeight,
                                _ownerDirection,
                                _widthSizingMode,
                                _heightSizingMode,
                                _ownerWidth,
                                _ownerHeight,
                                true,
                                facebook_yoga_LayoutPassReason_kStretch,
                                _layoutMarkerData,
                                _depth,
                                _generationCount,
                            )
                        });
                    }
                } else {
                    let mut remainingCrossDim: f32 = ({ containerCrossAxis } - {
                        ({
                            facebook_yoga_NodeImpl::dimensionWithMargin(
                                &child,
                                crossAxis,
                                availableInnerWidth,
                            )
                        })
                    });
                    if ({
                        facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            crossAxis,
                            direction,
                        )
                    }) && ({
                        facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            crossAxis,
                            direction,
                        )
                    }) {
                        leadingCrossDim +=
                            ({ maxOrDefined_110(0_f32, (remainingCrossDim / 2_f32)) });
                    } else if ({
                        facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            crossAxis,
                            direction,
                        )
                    }) {
                    } else if ({
                        facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            crossAxis,
                            direction,
                        )
                    }) {
                        leadingCrossDim += ({ maxOrDefined_110(0_f32, remainingCrossDim) });
                    } else if (alignItem == facebook_yoga_Align_FlexStart) {
                    } else if (alignItem == facebook_yoga_Align_Center) {
                        leadingCrossDim += (remainingCrossDim / 2_f32);
                    } else {
                        leadingCrossDim += remainingCrossDim;
                    }
                }
                ({
                    let _position: f32 = ({
                        ({
                            ({
                                facebook_yoga_LayoutResultsImpl::position(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                    ({ flexStartEdge_132(crossAxis) }),
                                )
                            })
                        } + { totalLineCrossDim })
                    } + { leadingCrossDim });
                    let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_132(crossAxis) });
                    facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
                });
            }
        }
        let mut appliedCrossGap: f32 = if (lineCount != 0_usize) {
            crossAxisGap
        } else {
            0_f32
        };
        totalLineCrossDim += ({ (*flexLine.borrow()).layout.crossDim } + appliedCrossGap);
        let __rhs = ({ maxOrDefined_110(maxLineMainDim, { (*flexLine.borrow()).layout.mainDim }) });
        maxLineMainDim = __rhs;
        lineCount.postfix_inc();
    }
    if (performLayout) && ((isNodeFlexWrap) || ({ isBaselineLayout_434((node).clone()) })) {
        let mut leadPerLine: f32 = 0_f32;
        let mut currentLead: f32 = leadingPaddingAndBorderCross;
        let mut extraSpacePerLine: f32 = 0_f32;
        let mut unclampedCrossDim: f32 =
            if (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit) {
                (availableInnerCrossDim + paddingAndBorderAxisCross)
            } else {
                if ({
                    facebook_yoga_NodeImpl::hasDefiniteLength(
                        &node,
                        ({ dimension_136(crossAxis) }),
                        crossAxisOwnerSize,
                    )
                }) {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({
                                    facebook_yoga_NodeImpl::getResolvedDimension(
                                        &node,
                                        direction,
                                        ({ dimension_136(crossAxis) }),
                                        crossAxisOwnerSize,
                                        ownerWidth,
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    })
                } else {
                    (totalLineCrossDim + paddingAndBorderAxisCross)
                }
            };
        let mut innerCrossDim: f32 = ({
            ({
                boundAxis_419(
                    (node).clone(),
                    crossAxis,
                    direction,
                    unclampedCrossDim,
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            })
        } - { paddingAndBorderAxisCross });
        let mut remainingAlignContentDim: f32 = (innerCrossDim - totalLineCrossDim);
        let mut alignContent: facebook_yoga_Align = if (remainingAlignContentDim >= 0_f32) {
            ({
                facebook_yoga_StyleImpl::alignContent(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            })
        } else {
            ({
                fallbackAlignment_415(
                    ({
                        facebook_yoga_StyleImpl::alignContent(
                            &({ facebook_yoga_NodeImpl::style_24(&node) }),
                        )
                    }),
                )
            })
        };
        'switch: {
            match { alignContent } {
                __v if __v == facebook_yoga_Align_Start || __v == facebook_yoga_Align_End => {
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_FlexEnd => {
                    currentLead += remainingAlignContentDim;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Center => {
                    currentLead += (remainingAlignContentDim / 2_f32);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Stretch => {
                    extraSpacePerLine = (remainingAlignContentDim / (lineCount as f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceAround => {
                    currentLead += (remainingAlignContentDim / (2_f32 * (lineCount as f32)));
                    leadPerLine = (remainingAlignContentDim / (lineCount as f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceEvenly => {
                    currentLead +=
                        (remainingAlignContentDim / (((lineCount).wrapping_add(1_usize)) as f32));
                    leadPerLine =
                        (remainingAlignContentDim / (((lineCount).wrapping_add(1_usize)) as f32));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceBetween => {
                    if (lineCount > 1_usize) {
                        leadPerLine = (remainingAlignContentDim
                            / (((lineCount).wrapping_sub(1_usize)) as f32));
                    }
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Auto
                    || __v == facebook_yoga_Align_FlexStart
                    || __v == facebook_yoga_Align_Baseline =>
                {
                    break 'switch;
                }
                _ => {}
            }
        };
        let endIterator: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(
                        &Rc::new(RefCell::new(
                            ({ facebook_yoga_NodeImpl::getLayoutChildren(&node) }),
                        ))
                        .as_pointer(),
                    )
                }),
            ));
        let mut i: usize = 0_usize;
        'loop_: while (i < lineCount) {
            let startIterator: Value<
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
            > = Rc::new(RefCell::new((*endIterator.borrow()).clone()));
            let iterator: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new((*startIterator.borrow()).clone()));
            let mut lineHeight: f32 = 0_f32;
            let mut maxAscentForCurrentLine: f32 = 0_f32;
            let mut maxDescentForCurrentLine: f32 = 0_f32;
            'loop_: while ({
                let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    iterator.as_pointer();
                let _b: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    Rc::new(RefCell::new(
                        ({
                            facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(
                                &Rc::new(RefCell::new(
                                    ({ facebook_yoga_NodeImpl::getLayoutChildren(&node) }),
                                ))
                                .as_pointer(),
                            )
                        }),
                    ));
                operator_ne_78(_a, _b.as_pointer())
            }) {
                let mut child: Ptr<facebook_yoga_Node> = ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_deref ( &iterator .as_pointer()  , )
                });
                if (({
                    facebook_yoga_StyleImpl::display(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    )
                }) == facebook_yoga_Display_None)
                {
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0  , )
                    });
                    continue 'loop_;
                }
                if (({
                    facebook_yoga_StyleImpl::positionType(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    )
                }) != facebook_yoga_PositionType_Absolute)
                {
                    if ({ ({ facebook_yoga_NodeImpl::getLineIndex(&child) }) } != { i }) {
                        break;
                    }
                    if ({ facebook_yoga_NodeImpl::isLayoutDimensionDefined(&child, crossAxis) }) {
                        let __rhs = ({
                            maxOrDefined_110(
                                lineHeight,
                                ({
                                    ({
                                        facebook_yoga_LayoutResultsImpl::measuredDimension(
                                            &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                            ({ dimension_136(crossAxis) }),
                                        )
                                    })
                                } + {
                                    ({
                                        facebook_yoga_StyleImpl::computeMarginForAxis(
                                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                            crossAxis,
                                            availableInnerWidth,
                                        )
                                    })
                                }),
                            )
                        });
                        lineHeight = __rhs;
                    }
                    if (({ resolveChildAlignment_413((node).clone(), (child).clone()) })
                        == facebook_yoga_Align_Baseline)
                    {
                        let mut ascent: f32 = ({ ({ calculateBaseline_433((child).clone()) }) }
                            + {
                                ({
                                    facebook_yoga_StyleImpl::computeFlexStartMargin(
                                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                        facebook_yoga_FlexDirection_Column,
                                        direction,
                                        availableInnerWidth,
                                    )
                                })
                            });
                        let mut descent: f32 = ({
                            ({
                                ({
                                    facebook_yoga_LayoutResultsImpl::measuredDimension(
                                        &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                        facebook_yoga_Dimension_Height,
                                    )
                                })
                            } + {
                                ({
                                    facebook_yoga_StyleImpl::computeMarginForAxis(
                                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                        facebook_yoga_FlexDirection_Column,
                                        availableInnerWidth,
                                    )
                                })
                            })
                        } - { ascent });
                        let __rhs = ({ maxOrDefined_110(maxAscentForCurrentLine, ascent) });
                        maxAscentForCurrentLine = __rhs;
                        let __rhs = ({ maxOrDefined_110(maxDescentForCurrentLine, descent) });
                        maxDescentForCurrentLine = __rhs;
                        let __rhs = ({
                            maxOrDefined_110(
                                lineHeight,
                                (maxAscentForCurrentLine + maxDescentForCurrentLine),
                            )
                        });
                        lineHeight = __rhs;
                    }
                }
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0  , )
                });
            }
            (*endIterator.borrow_mut()) = (*iterator.borrow()).clone();
            currentLead += if (i != 0_usize) { crossAxisGap } else { 0_f32 };
            lineHeight += extraSpacePerLine;
            (*iterator.borrow_mut()) = (*startIterator.borrow()).clone();
            'loop_: while ({
                let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    iterator.as_pointer();
                operator_ne_78(_a, endIterator.as_pointer())
            }) {
                let mut child: Ptr<facebook_yoga_Node> = ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_deref ( &iterator .as_pointer()  , )
                });
                if (({
                    facebook_yoga_StyleImpl::display(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    )
                }) == facebook_yoga_Display_None)
                {
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0  , )
                    });
                    continue 'loop_;
                }
                if (({
                    facebook_yoga_StyleImpl::positionType(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    )
                }) != facebook_yoga_PositionType_Absolute)
                {
                    'switch: {
                        match { ({ resolveChildAlignment_413((node).clone(), (child).clone()) }) } {
                            __v if __v == facebook_yoga_Align_Start
                                || __v == facebook_yoga_Align_End =>
                            {
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexStart => {
                                ({
                                    let _position: f32 = ({ currentLead } + {
                                        ({
                                            facebook_yoga_StyleImpl::computeFlexStartPosition(
                                                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                                crossAxis,
                                                direction,
                                                availableInnerWidth,
                                            )
                                        })
                                    });
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ flexStartEdge_132(crossAxis) });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child, _position, _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexEnd => {
                                ({
                                    let _position: f32 = ({
                                        ({ (currentLead + lineHeight) } - {
                                            ({
                                                facebook_yoga_StyleImpl::computeFlexEndMargin(
                                                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                                    crossAxis,
                                                    direction,
                                                    availableInnerWidth,
                                                )
                                            })
                                        })
                                    } - {
                                        ({
                                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                                ({ dimension_136(crossAxis) }),
                                            )
                                        })
                                    });
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ flexStartEdge_132(crossAxis) });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child, _position, _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Center => {
                                let mut childHeight: f32 = ({
                                    facebook_yoga_LayoutResultsImpl::measuredDimension(
                                        &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                        ({ dimension_136(crossAxis) }),
                                    )
                                });
                                ({
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child,
                                        (currentLead + ((lineHeight - childHeight) / 2_f32)),
                                        ({ flexStartEdge_132(crossAxis) }),
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Stretch => {
                                ({
                                    let _position: f32 = ({ currentLead } + {
                                        ({
                                            facebook_yoga_StyleImpl::computeFlexStartMargin(
                                                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                                crossAxis,
                                                direction,
                                                availableInnerWidth,
                                            )
                                        })
                                    });
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ flexStartEdge_132(crossAxis) });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child, _position, _edge,
                                    )
                                });
                                if !({
                                    facebook_yoga_NodeImpl::hasDefiniteLength(
                                        &child,
                                        ({ dimension_136(crossAxis) }),
                                        availableInnerCrossDim,
                                    )
                                }) {
                                    let mut childWidth: f32 = if isMainAxisRow {
                                        ({
                                            ({
                                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                                    &({
                                                        facebook_yoga_NodeImpl::getLayout_26(&child)
                                                    }),
                                                    facebook_yoga_Dimension_Width,
                                                )
                                            })
                                        } + {
                                            ({
                                                facebook_yoga_StyleImpl::computeMarginForAxis(
                                                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                                    mainAxis,
                                                    availableInnerWidth,
                                                )
                                            })
                                        })
                                    } else {
                                        (leadPerLine + lineHeight)
                                    };
                                    let mut childHeight: f32 = if !(isMainAxisRow) {
                                        ({
                                            ({
                                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                                    &({
                                                        facebook_yoga_NodeImpl::getLayout_26(&child)
                                                    }),
                                                    facebook_yoga_Dimension_Height,
                                                )
                                            })
                                        } + {
                                            ({
                                                facebook_yoga_StyleImpl::computeMarginForAxis(
                                                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                                    crossAxis,
                                                    availableInnerWidth,
                                                )
                                            })
                                        })
                                    } else {
                                        (leadPerLine + lineHeight)
                                    };
                                    if !(({
                                        inexactEquals_111(
                                            childWidth,
                                            ({
                                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                                    &({
                                                        facebook_yoga_NodeImpl::getLayout_26(&child)
                                                    }),
                                                    facebook_yoga_Dimension_Width,
                                                )
                                            }),
                                        )
                                    }) && ({
                                        inexactEquals_111(
                                            childHeight,
                                            ({
                                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                                    &({
                                                        facebook_yoga_NodeImpl::getLayout_26(&child)
                                                    }),
                                                    facebook_yoga_Dimension_Height,
                                                )
                                            }),
                                        )
                                    })) {
                                        ({
                                            let _node: Ptr<facebook_yoga_Node> = (child).clone();
                                            let _availableWidth: f32 = childWidth;
                                            let _availableHeight: f32 = childHeight;
                                            let _ownerDirection: facebook_yoga_Direction =
                                                direction;
                                            let _ownerWidth: f32 = availableInnerWidth;
                                            let _ownerHeight: f32 = availableInnerHeight;
                                            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> =
                                                (layoutMarkerData).clone();
                                            let _depth: u32 = depth;
                                            let _generationCount: u32 = generationCount;
                                            calculateLayoutInternal_430(
                                                _node,
                                                _availableWidth,
                                                _availableHeight,
                                                _ownerDirection,
                                                facebook_yoga_SizingMode_StretchFit,
                                                facebook_yoga_SizingMode_StretchFit,
                                                _ownerWidth,
                                                _ownerHeight,
                                                true,
                                                facebook_yoga_LayoutPassReason_kMultilineStretch,
                                                _layoutMarkerData,
                                                _depth,
                                                _generationCount,
                                            )
                                        });
                                    }
                                }
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Baseline => {
                                ({
                                    let _position: f32 = ({
                                        ({ (currentLead + maxAscentForCurrentLine) } - {
                                            ({ calculateBaseline_433((child).clone()) })
                                        })
                                    } + {
                                        ({
                                            facebook_yoga_StyleImpl::computeFlexStartPosition(
                                                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                                facebook_yoga_FlexDirection_Column,
                                                direction,
                                                availableInnerCrossDim,
                                            )
                                        })
                                    });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child,
                                        _position,
                                        facebook_yoga_PhysicalEdge_Top,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Auto
                                || __v == facebook_yoga_Align_SpaceBetween
                                || __v == facebook_yoga_Align_SpaceAround
                                || __v == facebook_yoga_Align_SpaceEvenly =>
                            {
                                break 'switch;
                            }
                            _ => {}
                        }
                    };
                }
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0  , )
                });
            }
            currentLead = { ((currentLead + leadPerLine) + lineHeight) };
            i.postfix_inc();
        }
    }
    ({
        let _measuredDimension: f32 = ({
            let _axisSize: f32 = ownerWidth;
            let _widthSize: f32 = ownerWidth;
            boundAxis_419(
                (node).clone(),
                facebook_yoga_FlexDirection_Row,
                direction,
                (availableWidth - marginAxisRow),
                _axisSize,
                _widthSize,
            )
        });
        facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
            &node,
            _measuredDimension,
            facebook_yoga_Dimension_Width,
        )
    });
    ({
        let _measuredDimension: f32 = ({
            boundAxis_419(
                (node).clone(),
                facebook_yoga_FlexDirection_Column,
                direction,
                (availableHeight - marginAxisColumn),
                ownerHeight,
                ownerWidth,
            )
        });
        facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
            &node,
            _measuredDimension,
            facebook_yoga_Dimension_Height,
        )
    });
    if (sizingModeMainDim == facebook_yoga_SizingMode_MaxContent)
        || ((({
            facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_24(&node) }))
        }) != facebook_yoga_Overflow_Scroll)
            && (sizingModeMainDim == facebook_yoga_SizingMode_FitContent))
    {
        ({
            let _measuredDimension: f32 = ({
                boundAxis_419(
                    (node).clone(),
                    mainAxis,
                    direction,
                    maxLineMainDim,
                    mainAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_136(mainAxis) });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                _dimension,
            )
        });
    } else if (sizingModeMainDim == facebook_yoga_SizingMode_FitContent)
        && (({ facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            == facebook_yoga_Overflow_Scroll)
    {
        ({
            let _measuredDimension: f32 = ({
                let _a: f32 = ({
                    minOrDefined_438(
                        (availableInnerMainDim + paddingAndBorderAxisMain),
                        ({
                            facebook_yoga_FloatOptionalImpl::unwrap(
                                &Rc::new(RefCell::new(
                                    ({
                                        boundAxisWithinMinAndMax_418(
                                            (node).clone(),
                                            direction,
                                            mainAxis,
                                            facebook_yoga_FloatOptional::new_1({ maxLineMainDim }),
                                            mainAxisOwnerSize,
                                            ownerWidth,
                                        )
                                    }),
                                ))
                                .as_pointer(),
                            )
                        }),
                    )
                });
                let _b: f32 = paddingAndBorderAxisMain;
                maxOrDefined_110(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_136(mainAxis) });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                _dimension,
            )
        });
    }
    if (sizingModeCrossDim == facebook_yoga_SizingMode_MaxContent)
        || ((({
            facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_24(&node) }))
        }) != facebook_yoga_Overflow_Scroll)
            && (sizingModeCrossDim == facebook_yoga_SizingMode_FitContent))
    {
        ({
            let _measuredDimension: f32 = ({
                boundAxis_419(
                    (node).clone(),
                    crossAxis,
                    direction,
                    (totalLineCrossDim + paddingAndBorderAxisCross),
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_136(crossAxis) });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                _dimension,
            )
        });
    } else if (sizingModeCrossDim == facebook_yoga_SizingMode_FitContent)
        && (({ facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            == facebook_yoga_Overflow_Scroll)
    {
        ({
            let _measuredDimension: f32 = ({
                let _a: f32 = ({
                    let _a: f32 = (availableInnerCrossDim + paddingAndBorderAxisCross);
                    let _b: f32 = ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({
                                    boundAxisWithinMinAndMax_418(
                                        (node).clone(),
                                        direction,
                                        crossAxis,
                                        facebook_yoga_FloatOptional::new_1({
                                            (totalLineCrossDim + paddingAndBorderAxisCross)
                                        }),
                                        crossAxisOwnerSize,
                                        ownerWidth,
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    });
                    minOrDefined_438(_a, _b)
                });
                let _b: f32 = paddingAndBorderAxisCross;
                maxOrDefined_110(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_136(crossAxis) });
            facebook_yoga_NodeImpl::setLayoutMeasuredDimension(
                &node,
                _measuredDimension,
                _dimension,
            )
        });
    }
    if (performLayout)
        && (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            == facebook_yoga_Wrap_WrapReverse)
    {
        let __tmp_2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
            RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
        );
        let __range3: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> =
            __tmp_2.as_pointer();
        let __begin3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(&__range3) }),
            ));
        let __end3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new(
                ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(&__range3) }),
            ));
        'loop_: while ({
            let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                __begin3.as_pointer();
            operator_ne_78(_a, __end3.as_pointer())
        }) {
            let mut child: Ptr<facebook_yoga_Node> = ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                    &__begin3.as_pointer(),
                )
            });
            if (({
                facebook_yoga_StyleImpl::positionType(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                )
            }) != facebook_yoga_PositionType_Absolute)
            {
                ({
                    let _position: f32 = ({
                        ({
                            ({
                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                                    ({ dimension_136(crossAxis) }),
                                )
                            })
                        } - {
                            ({
                                facebook_yoga_LayoutResultsImpl::position(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                    ({ flexStartEdge_132(crossAxis) }),
                                )
                            })
                        })
                    } - {
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                ({ dimension_136(crossAxis) }),
                            )
                        })
                    });
                    let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_132(crossAxis) });
                    facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
                });
            }
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin3.as_pointer(),
                )
            });
        }
    }
    if performLayout {
        let mut needsMainTrailingPos: bool = ({ needsTrailingPosition_422(mainAxis) });
        let mut needsCrossTrailingPos: bool = ({ needsTrailingPosition_422(crossAxis) });
        if (needsMainTrailingPos) || (needsCrossTrailingPos) {
            let __tmp_3: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
                RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
            );
            let __range4: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> =
                __tmp_3.as_pointer();
            let __begin4: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new(
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(&__range4)
                    }),
                ));
            let __end4: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new(
                    ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(&__range4) }),
                ));
            'loop_: while ({
                let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    __begin4.as_pointer();
                operator_ne_78(_a, __end4.as_pointer())
            }) {
                let mut child: Ptr<facebook_yoga_Node> = ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_deref ( &__begin4 .as_pointer()  , )
                });
                if (({
                    facebook_yoga_StyleImpl::display(
                        &({ facebook_yoga_NodeImpl::style_24(&child) }),
                    )
                }) == facebook_yoga_Display_None)
                    || (({
                        facebook_yoga_StyleImpl::positionType(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                        )
                    }) == facebook_yoga_PositionType_Absolute)
                {
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_inc_4 ( &__begin4 .as_pointer()  , )
                    });
                    continue 'loop_;
                }
                if needsMainTrailingPos {
                    ({ setChildTrailingPosition_421((node).clone(), (child).clone(), mainAxis) });
                }
                if needsCrossTrailingPos {
                    ({ setChildTrailingPosition_421((node).clone(), (child).clone(), crossAxis) });
                }
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_inc_4 ( &__begin4 .as_pointer()  , )
                });
            }
        }
        if ((({
            facebook_yoga_StyleImpl::positionType(&({ facebook_yoga_NodeImpl::style_24(&node) }))
        }) != facebook_yoga_PositionType_Static)
            || ({ facebook_yoga_NodeImpl::alwaysFormsContainingBlock(&node) }))
            || (depth == 1_u32)
        {
            ({
                let _containingNode: Ptr<facebook_yoga_Node> = (node).clone();
                let _currentNode: Ptr<facebook_yoga_Node> = (node).clone();
                let _widthSizingMode: facebook_yoga_SizingMode = if isMainAxisRow {
                    sizingModeMainDim
                } else {
                    sizingModeCrossDim
                };
                let _currentNodeDirection: facebook_yoga_Direction = direction;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _currentDepth: u32 = depth;
                let _generationCount: u32 = generationCount;
                let _containingNodeAvailableInnerWidth: f32 = availableInnerWidth;
                let _containingNodeAvailableInnerHeight: f32 = availableInnerHeight;
                layoutAbsoluteDescendants_431(
                    _containingNode,
                    _currentNode,
                    _widthSizingMode,
                    _currentNodeDirection,
                    _layoutMarkerData,
                    _currentDepth,
                    _generationCount,
                    0_f32,
                    0_f32,
                    _containingNodeAvailableInnerWidth,
                    _containingNodeAvailableInnerHeight,
                )
            });
        }
    }
}
pub fn isCachedForOwnerSize_467(
    mut node: Ptr<facebook_yoga_Node>,
    cachedMeasurement: Ptr<facebook_yoga_CachedMeasurement>,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
) -> bool {
    return ((!({
        facebook_yoga_StyleImpl::resolvesAgainstOwnerWidth(
            &({ facebook_yoga_NodeImpl::style_25(&node) }),
        )
    })) || ({
        inexactEquals_111(cachedMeasurement.with(|__s| __s.ownerWidth), ownerWidth)
    })) && ((!({
        facebook_yoga_StyleImpl::resolvesAgainstOwnerHeight(
            &({ facebook_yoga_NodeImpl::style_25(&node) }),
        )
    })) || ({
        inexactEquals_111(cachedMeasurement.with(|__s| __s.ownerHeight), ownerHeight)
    }));
}
pub fn calculateLayoutInternal_430(
    mut node: Ptr<facebook_yoga_Node>,
    mut availableWidth: f32,
    mut availableHeight: f32,
    mut ownerDirection: facebook_yoga_Direction,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut performLayout: bool,
    mut reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: u32,
    mut generationCount: u32,
) -> bool {
    let mut layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    depth.postfix_inc();
    let mut needToVisitNode: bool = ((({ facebook_yoga_NodeImpl::isDirty(&node) })
        && ({ layout.with(|__s| __s.generationCount) } != { generationCount }))
        || ({ layout.with(|__s| __s.configVersion) } != {
            ({
                facebook_yoga_ConfigImpl::getVersion(
                    &({ facebook_yoga_NodeImpl::getConfig(&node) }),
                )
            })
        }))
        || ({ layout.with(|__s| __s.lastOwnerDirection) } != { ownerDirection });
    if needToVisitNode {
        field!(layout, nextCachedMeasurementsIndex).write(0_u32);
        field!(field!(layout, cachedLayout), availableWidth).write((-1_i32 as f32));
        field!(field!(layout, cachedLayout), availableHeight).write((-1_i32 as f32));
        field!(field!(layout, cachedLayout), widthSizingMode)
            .write(facebook_yoga_SizingMode_MaxContent);
        field!(field!(layout, cachedLayout), heightSizingMode)
            .write(facebook_yoga_SizingMode_MaxContent);
        field!(field!(layout, cachedLayout), computedWidth).write((-1_i32 as f32));
        field!(field!(layout, cachedLayout), computedHeight).write((-1_i32 as f32));
        field!(field!(layout, cachedLayout), ownerWidth).write((-1_i32 as f32));
        field!(field!(layout, cachedLayout), ownerHeight).write((-1_i32 as f32));
    }
    let mut cachedResults: Ptr<facebook_yoga_CachedMeasurement> =
        Ptr::<facebook_yoga_CachedMeasurement>::null();
    if ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) }) {
        let mut marginAxisRow: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let mut marginAxisColumn: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        });
        if ({
            let _lastWidthMode: facebook_yoga_SizingMode =
                layout.with(|__s| __s.cachedLayout.widthSizingMode);
            let _lastAvailableWidth: f32 = layout.with(|__s| __s.cachedLayout.availableWidth);
            let _lastHeightMode: facebook_yoga_SizingMode =
                layout.with(|__s| __s.cachedLayout.heightSizingMode);
            let _lastAvailableHeight: f32 = layout.with(|__s| __s.cachedLayout.availableHeight);
            let _lastComputedWidth: f32 = layout.with(|__s| __s.cachedLayout.computedWidth);
            let _lastComputedHeight: f32 = layout.with(|__s| __s.cachedLayout.computedHeight);
            canUseCachedMeasurement_242(
                widthSizingMode,
                availableWidth,
                heightSizingMode,
                availableHeight,
                _lastWidthMode,
                _lastAvailableWidth,
                _lastHeightMode,
                _lastAvailableHeight,
                _lastComputedWidth,
                _lastComputedHeight,
                marginAxisRow,
                marginAxisColumn,
                ({ facebook_yoga_NodeImpl::getConfig(&node) }),
            )
        }) && ({
            isCachedForOwnerSize_467(
                (node).clone(),
                field_ptr!(layout, cachedLayout),
                ownerWidth,
                ownerHeight,
            )
        }) {
            cachedResults = (field_ptr!(layout, cachedLayout));
        } else {
            let mut i: usize = 0_usize;
            'loop_: while ({ i } < {
                (layout.with(|__s| __s.nextCachedMeasurementsIndex) as usize)
            }) {
                if ({
                    let _lastWidthMode: facebook_yoga_SizingMode =
                        field_ref!(layout, cachedMeasurements)
                            .decay()
                            .offset(i as isize)
                            .with(|__s| __s.widthSizingMode);
                    let _lastAvailableWidth: f32 = field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset(i as isize)
                        .with(|__s| __s.availableWidth);
                    let _lastHeightMode: facebook_yoga_SizingMode =
                        field_ref!(layout, cachedMeasurements)
                            .decay()
                            .offset(i as isize)
                            .with(|__s| __s.heightSizingMode);
                    let _lastAvailableHeight: f32 = field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset(i as isize)
                        .with(|__s| __s.availableHeight);
                    let _lastComputedWidth: f32 = field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset(i as isize)
                        .with(|__s| __s.computedWidth);
                    let _lastComputedHeight: f32 = field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset(i as isize)
                        .with(|__s| __s.computedHeight);
                    canUseCachedMeasurement_242(
                        widthSizingMode,
                        availableWidth,
                        heightSizingMode,
                        availableHeight,
                        _lastWidthMode,
                        _lastAvailableWidth,
                        _lastHeightMode,
                        _lastAvailableHeight,
                        _lastComputedWidth,
                        _lastComputedHeight,
                        marginAxisRow,
                        marginAxisColumn,
                        ({ facebook_yoga_NodeImpl::getConfig(&node) }),
                    )
                }) && ({
                    isCachedForOwnerSize_467(
                        (node).clone(),
                        field_ref!(layout, cachedMeasurements)
                            .decay()
                            .offset(i as isize),
                        ownerWidth,
                        ownerHeight,
                    )
                }) {
                    cachedResults = (field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset(i as isize));
                    break;
                }
                i.postfix_inc();
            }
        }
    } else if performLayout {
        if (((({
            inexactEquals_111(
                layout.with(|__s| __s.cachedLayout.availableWidth),
                availableWidth,
            )
        }) && ({
            inexactEquals_111(
                layout.with(|__s| __s.cachedLayout.availableHeight),
                availableHeight,
            )
        })) && ({ layout.with(|__s| __s.cachedLayout.widthSizingMode) } == {
            widthSizingMode
        })) && ({ layout.with(|__s| __s.cachedLayout.heightSizingMode) } == {
            heightSizingMode
        })) && ({
            isCachedForOwnerSize_467(
                (node).clone(),
                field_ptr!(layout, cachedLayout),
                ownerWidth,
                ownerHeight,
            )
        }) {
            cachedResults = (field_ptr!(layout, cachedLayout));
        }
    } else {
        let mut i: u32 = 0_u32;
        'loop_: while ({ i } < { layout.with(|__s| __s.nextCachedMeasurementsIndex) }) {
            if (((({
                inexactEquals_111(
                    field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset((i as usize) as isize)
                        .with(|__s| __s.availableWidth),
                    availableWidth,
                )
            }) && ({
                inexactEquals_111(
                    field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset((i as usize) as isize)
                        .with(|__s| __s.availableHeight),
                    availableHeight,
                )
            })) && ({
                field_ref!(layout, cachedMeasurements)
                    .decay()
                    .offset((i as usize) as isize)
                    .with(|__s| __s.widthSizingMode)
            } == { widthSizingMode }))
                && ({
                    field_ref!(layout, cachedMeasurements)
                        .decay()
                        .offset((i as usize) as isize)
                        .with(|__s| __s.heightSizingMode)
                } == { heightSizingMode }))
                && ({
                    isCachedForOwnerSize_467(
                        (node).clone(),
                        field_ref!(layout, cachedMeasurements)
                            .decay()
                            .offset((i as usize) as isize),
                        ownerWidth,
                        ownerHeight,
                    )
                })
            {
                cachedResults = (field_ref!(layout, cachedMeasurements)
                    .decay()
                    .offset((i as usize) as isize));
                break;
            }
            i.postfix_inc();
        }
    }
    if (!(needToVisitNode)) && (!((cachedResults).is_null())) {
        ({
            facebook_yoga_LayoutResultsImpl::setMeasuredDimension(
                &layout,
                facebook_yoga_Dimension_Width,
                cachedResults.with(|__s| __s.computedWidth),
            )
        });
        ({
            facebook_yoga_LayoutResultsImpl::setMeasuredDimension(
                &layout,
                facebook_yoga_Dimension_Height,
                cachedResults.with(|__s| __s.computedHeight),
            )
        });
        {
            if performLayout {
                (field_ptr!(layoutMarkerData, cachedLayouts))
            } else {
                (field_ptr!(layoutMarkerData, cachedMeasures))
            }
            .with_mut(|__v| *__v = *__v + 1)
        };
    } else {
        ({
            let _node: Ptr<facebook_yoga_Node> = (node).clone();
            let _availableWidth: f32 = availableWidth;
            let _availableHeight: f32 = availableHeight;
            let _ownerDirection: facebook_yoga_Direction = ownerDirection;
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: f32 = ownerWidth;
            let _ownerHeight: f32 = ownerHeight;
            let _performLayout: bool = performLayout;
            let _reason: facebook_yoga_LayoutPassReason = reason;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: u32 = depth;
            let _generationCount: u32 = generationCount;
            calculateLayoutImpl_465(
                _node,
                _availableWidth,
                _availableHeight,
                _ownerDirection,
                _widthSizingMode,
                _heightSizingMode,
                _ownerWidth,
                _ownerHeight,
                _performLayout,
                _reason,
                _layoutMarkerData,
                _depth,
                _generationCount,
            )
        });
        field!(layout, lastOwnerDirection).write(ownerDirection);
        let __rhs = ({
            facebook_yoga_ConfigImpl::getVersion(&({ facebook_yoga_NodeImpl::getConfig(&node) }))
        });
        field!(layout, configVersion).write(__rhs);
        if (cachedResults).is_null() {
            let __rhs = {
                let __tmp_4: Value<u32> = Rc::new(RefCell::new(
                    (layout.with(|__s| __s.nextCachedMeasurementsIndex))
                        .wrapping_add((1_u32 as u32)),
                ));
                ({
                    let __a = field_ptr!(layoutMarkerData, maxMeasureCache);
                    let __b = __tmp_4.as_pointer();
                    if CppLess::less(&__a, &__b) { __b } else { __a }
                }
                .read())
            };
            field!(layoutMarkerData, maxMeasureCache).write(__rhs);
            if ({ layout.with(|__s| __s.nextCachedMeasurementsIndex) } == {
                (MaxCachedMeasurements_123.with(|rc| *rc.borrow()) as u32)
            }) {
                field!(layout, nextCachedMeasurementsIndex).write(0_u32);
            }
            let mut newCacheEntry: Ptr<facebook_yoga_CachedMeasurement> =
                Ptr::<facebook_yoga_CachedMeasurement>::null();
            if performLayout {
                newCacheEntry = (field_ptr!(layout, cachedLayout));
            } else {
                newCacheEntry = (field_ref!(layout, cachedMeasurements).decay().offset(
                    (layout.with(|__s| __s.nextCachedMeasurementsIndex) as usize) as isize,
                ));
                field!(layout, nextCachedMeasurementsIndex).with_mut(|__v| __v.postfix_inc());
            }
            field!(newCacheEntry, availableWidth).write(availableWidth);
            field!(newCacheEntry, availableHeight).write(availableHeight);
            field!(newCacheEntry, widthSizingMode).write(widthSizingMode);
            field!(newCacheEntry, heightSizingMode).write(heightSizingMode);
            let __rhs = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &layout,
                    facebook_yoga_Dimension_Width,
                )
            });
            field!(newCacheEntry, computedWidth).write(__rhs);
            let __rhs = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &layout,
                    facebook_yoga_Dimension_Height,
                )
            });
            field!(newCacheEntry, computedHeight).write(__rhs);
            field!(newCacheEntry, ownerWidth).write(ownerWidth);
            field!(newCacheEntry, ownerHeight).write(ownerHeight);
        }
    }
    if performLayout {
        ({
            let _lengthValue: f32 = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                    facebook_yoga_Dimension_Width,
                )
            });
            facebook_yoga_NodeImpl::setLayoutDimension(
                &node,
                _lengthValue,
                facebook_yoga_Dimension_Width,
            )
        });
        ({
            let _lengthValue: f32 = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                    facebook_yoga_Dimension_Height,
                )
            });
            facebook_yoga_NodeImpl::setLayoutDimension(
                &node,
                _lengthValue,
                facebook_yoga_Dimension_Height,
            )
        });
        ({ facebook_yoga_NodeImpl::setHasNewLayout(&node, true) });
        ({ facebook_yoga_NodeImpl::setDirty(&node, false) });
    }
    field!(layout, generationCount).write(generationCount);
    let mut layoutType: facebook_yoga_LayoutType = <facebook_yoga_LayoutType>::default();
    if performLayout {
        layoutType = if (!(needToVisitNode))
            && ({ (cachedResults).clone() } == { (field_ptr!(layout, cachedLayout)) })
        {
            facebook_yoga_LayoutType_kCachedLayout
        } else {
            facebook_yoga_LayoutType_kLayout
        };
    } else {
        layoutType = if !((cachedResults).is_null()) {
            facebook_yoga_LayoutType_kCachedMeasure
        } else {
            facebook_yoga_LayoutType_kMeasure
        };
    }
    ({
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeLayout_ {
                    layoutType: layoutType,
                },
            ));
        facebook_yoga_Event::publish_3_2(node.reinterpret_cast::<YGNode>(), _eventData.as_pointer())
    });
    return ((needToVisitNode) || ((cachedResults).is_null()));
}
pub fn calculateLayout_208(
    mut node: Ptr<facebook_yoga_Node>,
    mut ownerWidth: f32,
    mut ownerHeight: f32,
    mut ownerDirection: facebook_yoga_Direction,
) {
    ({ facebook_yoga_Event::publish_3_3(node.reinterpret_cast::<YGNode>(), None) });
    let markerData: Value<facebook_yoga_LayoutData> =
        Rc::new(RefCell::new(<facebook_yoga_LayoutData>::default()));
    let mut currentGenerationCount: u32 = ({
        let __old = gCurrentGenerationCount_439.with(|rc| *rc.borrow());
        (*gCurrentGenerationCount_439.with(Value::clone).borrow_mut()) = __old.wrapping_add(1_u32);
        __old
    })
    .wrapping_add(1_u32);
    ({ facebook_yoga_NodeImpl::processDimensions(&node) });
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    let mut width: f32 = YGUndefined_23.with(|rc| *rc.borrow());
    let mut widthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&node) });
    if ({
        facebook_yoga_NodeImpl::hasDefiniteLength(&node, facebook_yoga_Dimension_Width, ownerWidth)
    }) {
        width = ({
            ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({
                            let _referenceLength: f32 = ownerWidth;
                            let _ownerWidth: f32 = ownerWidth;
                            facebook_yoga_NodeImpl::getResolvedDimension(
                                &node,
                                direction,
                                ({ dimension_136(facebook_yoga_FlexDirection_Row) }),
                                _referenceLength,
                                _ownerWidth,
                            )
                        }),
                    ))
                    .as_pointer(),
                )
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeMarginForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    facebook_yoga_FlexDirection_Row,
                    ownerWidth,
                )
            })
        });
        widthSizingMode = facebook_yoga_SizingMode_StretchFit;
    } else if ({
        facebook_yoga_FloatOptionalImpl::isDefined(
            &Rc::new(RefCell::new(
                ({
                    let _direction: facebook_yoga_Direction = direction;
                    let _referenceLength: f32 = ownerWidth;
                    let _ownerWidth: f32 = ownerWidth;
                    facebook_yoga_StyleImpl::resolvedMaxDimension(
                        &style,
                        _direction,
                        facebook_yoga_Dimension_Width,
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            ))
            .as_pointer(),
        )
    }) {
        width = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: f32 = ownerWidth;
                        let _ownerWidth: f32 = ownerWidth;
                        facebook_yoga_StyleImpl::resolvedMaxDimension(
                            &style,
                            _direction,
                            facebook_yoga_Dimension_Width,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
        widthSizingMode = facebook_yoga_SizingMode_FitContent;
    } else {
        width = ownerWidth;
        widthSizingMode = if ({ isUndefined_105(width) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let mut height: f32 = YGUndefined_23.with(|rc| *rc.borrow());
    let mut heightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    if ({
        facebook_yoga_NodeImpl::hasDefiniteLength(
            &node,
            facebook_yoga_Dimension_Height,
            ownerHeight,
        )
    }) {
        height = ({
            ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({
                            facebook_yoga_NodeImpl::getResolvedDimension(
                                &node,
                                direction,
                                ({ dimension_136(facebook_yoga_FlexDirection_Column) }),
                                ownerHeight,
                                ownerWidth,
                            )
                        }),
                    ))
                    .as_pointer(),
                )
            })
        } + {
            ({
                facebook_yoga_StyleImpl::computeMarginForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    facebook_yoga_FlexDirection_Column,
                    ownerWidth,
                )
            })
        });
        heightSizingMode = facebook_yoga_SizingMode_StretchFit;
    } else if ({
        facebook_yoga_FloatOptionalImpl::isDefined(
            &Rc::new(RefCell::new(
                ({
                    let _direction: facebook_yoga_Direction = direction;
                    let _referenceLength: f32 = ownerHeight;
                    let _ownerWidth: f32 = ownerWidth;
                    facebook_yoga_StyleImpl::resolvedMaxDimension(
                        &style,
                        _direction,
                        facebook_yoga_Dimension_Height,
                        _referenceLength,
                        _ownerWidth,
                    )
                }),
            ))
            .as_pointer(),
        )
    }) {
        height = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _direction: facebook_yoga_Direction = direction;
                        let _referenceLength: f32 = ownerHeight;
                        let _ownerWidth: f32 = ownerWidth;
                        facebook_yoga_StyleImpl::resolvedMaxDimension(
                            &style,
                            _direction,
                            facebook_yoga_Dimension_Height,
                            _referenceLength,
                            _ownerWidth,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
        heightSizingMode = facebook_yoga_SizingMode_FitContent;
    } else {
        height = ownerHeight;
        heightSizingMode = if ({ isUndefined_105(height) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let mut generationCount: u32 = if ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    }) {
        currentGenerationCount
    } else {
        gCurrentGenerationCount_439.with(|rc| *rc.borrow())
    };
    if ({
        calculateLayoutInternal_430(
            (node).clone(),
            width,
            height,
            ownerDirection,
            widthSizingMode,
            heightSizingMode,
            ownerWidth,
            ownerHeight,
            true,
            facebook_yoga_LayoutPassReason_kInitial,
            markerData.as_pointer(),
            0_u32,
            generationCount,
        )
    }) {
        ({
            let _direction: facebook_yoga_Direction = ({
                facebook_yoga_LayoutResultsImpl::direction(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                )
            });
            facebook_yoga_NodeImpl::setPosition(&node, _direction, ownerWidth, ownerHeight)
        });
        ({ roundLayoutResultsToPixelGrid_468((node).clone(), (0_f32 as f64), (0_f32 as f64)) });
    }
    ({
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ {
                    layoutData: (markerData.as_pointer()),
                },
            ));
        facebook_yoga_Event::publish_3_4(node.reinterpret_cast::<YGNode>(), _eventData.as_pointer())
    });
}
pub fn calculateFlexLine_466(
    mut node: Ptr<facebook_yoga_Node>,
    mut ownerDirection: facebook_yoga_Direction,
    mut ownerWidth: f32,
    mut mainAxisOwnerSize: f32,
    mut availableInnerWidth: f32,
    mut availableInnerMainDim: f32,
    iterator: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    mut lineCount: usize,
) -> facebook_yoga_FlexLine {
    let itemsInFlow: Value<Vec<Ptr<facebook_yoga_Node>>> = Rc::new(RefCell::new(Vec::new()));
    match ({ ({ facebook_yoga_NodeImpl::getChildCount(&node) }) },) {
        (mut __tmp_0,) => {
            if __tmp_0 as usize > (*itemsInFlow.borrow()).capacity() as usize {
                let __len = (*itemsInFlow.borrow()).len();
                (*itemsInFlow.borrow_mut()).reserve_exact(__tmp_0 as usize - __len as usize);
            }
        }
    };
    let mut sizeConsumed: f32 = 0_f32;
    let mut totalFlexGrowFactors: f32 = 0_f32;
    let mut totalFlexShrinkScaledFactors: f32 = 0_f32;
    let mut numberOfAutoMargins: usize = 0_usize;
    let mut firstElementInLine: Ptr<facebook_yoga_Node> = Ptr::<facebook_yoga_Node>::null();
    let mut sizeConsumedIncludingMinConstraint: f32 = 0_f32;
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    let mut mainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_130(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            }),
            direction,
        )
    });
    let mut isNodeFlexWrap: bool =
        (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            != facebook_yoga_Wrap_NoWrap);
    let mut gap: f32 = ({
        facebook_yoga_StyleImpl::computeGapForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            availableInnerMainDim,
        )
    });
    let childrenEnd: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
        Rc::new(RefCell::new(
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_NodeImpl::getLayoutChildren(&node) }),
                    ))
                    .as_pointer(),
                )
            }),
        ));
    'loop_: while ({
        let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            (iterator).clone();
        let _b: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            childrenEnd.as_pointer();
        operator_ne_78(_a, _b)
    }) {
        let mut child: Ptr<facebook_yoga_Node> = ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                &iterator,
            )
        });
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
            == facebook_yoga_Display_None)
            || (({
                facebook_yoga_StyleImpl::positionType(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                )
            }) == facebook_yoga_PositionType_Absolute)
        {
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator  , 0  , )
            });
            continue 'loop_;
        }
        if (firstElementInLine).is_null() {
            firstElementInLine = (child).clone();
        }
        if ({
            facebook_yoga_StyleImpl::flexStartMarginIsAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                mainAxis,
                ownerDirection,
            )
        }) {
            numberOfAutoMargins.postfix_inc();
        }
        if ({
            facebook_yoga_StyleImpl::flexEndMarginIsAuto(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                mainAxis,
                ownerDirection,
            )
        }) {
            numberOfAutoMargins.postfix_inc();
        }
        ({ facebook_yoga_NodeImpl::setLineIndex(&child, lineCount) });
        let mut childMarginMainAxis: f32 = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                mainAxis,
                availableInnerWidth,
            )
        });
        let mut childLeadingGapMainAxis: f32 =
            if ({ (child).clone() } == { (firstElementInLine).clone() }) {
                0_f32
            } else {
                gap
            };
        let mut flexBasisWithMinAndMaxConstraints: f32 = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _node: Ptr<facebook_yoga_Node> = (child).clone();
                        let _value: facebook_yoga_FloatOptional =
                            ({ facebook_yoga_NodeImpl::getLayout_26(&child) })
                                .with(|__s| __s.computedFlexBasis.clone());
                        boundAxisWithinMinAndMax_418(
                            _node,
                            direction,
                            mainAxis,
                            _value,
                            mainAxisOwnerSize,
                            ownerWidth,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
        let mut requiredMainDim: f32 = (((sizeConsumedIncludingMinConstraint
            + flexBasisWithMinAndMaxConstraints)
            + childMarginMainAxis)
            + childLeadingGapMainAxis);
        if (((requiredMainDim > availableInnerMainDim)
            && (!({ inexactEquals_111(requiredMainDim, availableInnerMainDim) })))
            && (isNodeFlexWrap))
            && (!((*itemsInFlow.borrow()).is_empty()))
        {
            break;
        }
        sizeConsumedIncludingMinConstraint +=
            ((flexBasisWithMinAndMaxConstraints + childMarginMainAxis) + childLeadingGapMainAxis);
        sizeConsumed +=
            ((flexBasisWithMinAndMaxConstraints + childMarginMainAxis) + childLeadingGapMainAxis);
        if ({ facebook_yoga_NodeImpl::isNodeFlexible(&child) }) {
            totalFlexGrowFactors += ({ facebook_yoga_NodeImpl::resolveFlexGrow(&child) });
            totalFlexShrinkScaledFactors +=
                ({ -({ facebook_yoga_NodeImpl::resolveFlexShrink(&child) }) } * {
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!(
                            ({ facebook_yoga_NodeImpl::getLayout_26(&child,) }),
                            computedFlexBasis
                        ))
                    })
                });
        }
        (*itemsInFlow.borrow_mut()).push(child.clone());
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_post_inc_5(
                &iterator, 0,
            )
        });
    }
    if (totalFlexGrowFactors > 0_f32) && (totalFlexGrowFactors < 1_f32) {
        totalFlexGrowFactors = 1_f32;
    }
    if (totalFlexShrinkScaledFactors > 0_f32) && (totalFlexShrinkScaledFactors < 1_f32) {
        totalFlexShrinkScaledFactors = 1_f32;
    }
    return facebook_yoga_FlexLine {
        itemsInFlow: ({ std::mem::take(&mut (*itemsInFlow.borrow_mut())) }),
        sizeConsumed: sizeConsumed,
        numberOfAutoMargins: numberOfAutoMargins,
        layout: facebook_yoga_FlexLineRunningLayout {
            totalFlexGrowFactors: totalFlexGrowFactors,
            totalFlexShrinkScaledFactors: totalFlexShrinkScaledFactors,
            remainingFreeSpace: 0_f32,
            mainDim: 0_f32,
            crossDim: 0_f32,
        },
    };
}
pub fn roundValueToPixelGridDouble_469(
    mut value: f64,
    mut pointScaleFactor: f64,
    mut forceCeil: bool,
    mut forceFloor: bool,
) -> f64 {
    let mut scaledValue: f64 = (value * pointScaleFactor);
    let mut fractional: f64 = (scaledValue - scaledValue.trunc()).copysign(scaledValue);
    if (fractional < 0_f64) {
        fractional.prefix_inc();
    }
    if ({ inexactEquals_112(fractional, 0_f64) }) {
        scaledValue = { (scaledValue - fractional) };
    } else if ({ inexactEquals_112(fractional, 1_f64) }) {
        scaledValue = { ((scaledValue - fractional) + 1_f64) };
    } else if forceCeil {
        scaledValue = { ((scaledValue - fractional) + 1_f64) };
    } else if forceFloor {
        scaledValue = { (scaledValue - fractional) };
    } else {
        let __rhs = ((scaledValue - fractional)
            + (if (!(fractional.is_nan()))
                && ((fractional > 0.5_f64) || ({ inexactEquals_112(fractional, 0.5_f64) }))
            {
                1_f64
            } else {
                0_f64
            }));
        scaledValue = __rhs;
    }
    return if ((scaledValue.is_nan()) || (pointScaleFactor.is_nan())) {
        (YGUndefined_23.with(|rc| *rc.borrow()) as f64)
    } else {
        (scaledValue / pointScaleFactor)
    };
}
pub fn roundValueToPixelGrid_411(
    mut value: f64,
    mut pointScaleFactor: f64,
    mut forceCeil: bool,
    mut forceFloor: bool,
) -> f32 {
    return (({ roundValueToPixelGridDouble_469(value, pointScaleFactor, forceCeil, forceFloor) })
        as f32);
}
pub fn roundLayoutResultsToPixelGrid_468(
    mut node: Ptr<facebook_yoga_Node>,
    mut absoluteLeft: f64,
    mut absoluteTop: f64,
) {
    let mut pointScaleFactor: f64 = (({
        facebook_yoga_ConfigImpl::getPointScaleFactor(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
        )
    }) as f64);
    let mut nodeLeft: f64 = (({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_PhysicalEdge_Left,
        )
    }) as f64);
    let mut nodeTop: f64 = (({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_PhysicalEdge_Top,
        )
    }) as f64);
    let mut nodeWidth: f64 = (({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_Dimension_Width,
        )
    }) as f64);
    let mut nodeHeight: f64 = (({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_Dimension_Height,
        )
    }) as f64);
    let mut absoluteNodeLeft: f64 = (absoluteLeft + nodeLeft);
    let mut absoluteNodeTop: f64 = (absoluteTop + nodeTop);
    let mut absoluteNodeRight: f64 = (absoluteNodeLeft + nodeWidth);
    let mut absoluteNodeBottom: f64 = (absoluteNodeTop + nodeHeight);
    if (pointScaleFactor != 0_f64) {
        let mut textRounding: bool =
            (({ facebook_yoga_NodeImpl::getNodeType(&node) }) == facebook_yoga_NodeType_Text);
        ({
            facebook_yoga_NodeImpl::setLayoutPosition(
                &node,
                ({ roundValueToPixelGrid_411(nodeLeft, pointScaleFactor, false, textRounding) }),
                facebook_yoga_PhysicalEdge_Left,
            )
        });
        ({
            facebook_yoga_NodeImpl::setLayoutPosition(
                &node,
                ({ roundValueToPixelGrid_411(nodeTop, pointScaleFactor, false, textRounding) }),
                facebook_yoga_PhysicalEdge_Top,
            )
        });
        let mut scaledNodeWidth: f64 = (nodeWidth * pointScaleFactor);
        let mut hasFractionalWidth: bool = !({
            let _a: f64 = scaledNodeWidth.round();
            let _b: f64 = scaledNodeWidth;
            inexactEquals_112(_a, _b)
        });
        let mut scaledNodeHeight: f64 = (nodeHeight * pointScaleFactor);
        let mut hasFractionalHeight: bool = !({
            let _a: f64 = scaledNodeHeight.round();
            let _b: f64 = scaledNodeHeight;
            inexactEquals_112(_a, _b)
        });
        ({
            facebook_yoga_LayoutResultsImpl::setDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                facebook_yoga_Dimension_Width,
                if (textRounding) && (!(hasFractionalWidth)) {
                    ((scaledNodeWidth.round() / pointScaleFactor) as f32)
                } else {
                    ((({
                        roundValueToPixelGridDouble_469(
                            absoluteNodeRight,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - ({
                        roundValueToPixelGridDouble_469(
                            absoluteNodeLeft,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    })) as f32)
                },
            )
        });
        ({
            facebook_yoga_LayoutResultsImpl::setDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                facebook_yoga_Dimension_Height,
                if (textRounding) && (!(hasFractionalHeight)) {
                    ((scaledNodeHeight.round() / pointScaleFactor) as f32)
                } else {
                    ((({
                        roundValueToPixelGridDouble_469(
                            absoluteNodeBottom,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - ({
                        roundValueToPixelGridDouble_469(
                            absoluteNodeTop,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    })) as f32)
                },
            )
        });
    }
    let __range2: Ptr<Vec<Ptr<facebook_yoga_Node>>> =
        ({ facebook_yoga_NodeImpl::getChildren(&node) });
    'loop_: for child in IterRange::new(__range2.decay(), __range2.decay().to_end()) {
        let mut child: Ptr<facebook_yoga_Node> = (child.read());
        if ({ ({ facebook_yoga_NodeImpl::getOwner(&child) }) } != { (node).clone() }) {
            continue 'loop_;
        }
        ({ roundLayoutResultsToPixelGrid_468((child).clone(), absoluteNodeLeft, absoluteNodeTop) });
    }
}
pub fn configUpdateInvalidatesLayout_470(
    oldConfig: Ptr<facebook_yoga_Config>,
    newConfig: Ptr<facebook_yoga_Config>,
) -> bool {
    return ((({ ({ facebook_yoga_ConfigImpl::getErrata(&oldConfig) }) } != {
        ({ facebook_yoga_ConfigImpl::getErrata(&newConfig) })
    }) || (!(({ facebook_yoga_ConfigImpl::getEnabledExperiments(&oldConfig) })
        == ({ facebook_yoga_ConfigImpl::getEnabledExperiments(&newConfig) }))))
        || ({ ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&oldConfig) }) } != {
            ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&newConfig) })
        }))
        || ({ (({ facebook_yoga_ConfigImpl::useWebDefaults(&oldConfig) }) as i32) } != {
            (({ facebook_yoga_ConfigImpl::useWebDefaults(&newConfig) }) as i32)
        });
}
pub fn fatalWithMessage_103(mut message: Ptr<i8>) {
    message.to_rust_string();
}
pub fn assertFatal_201(mut condition: bool, mut message: Ptr<i8>) {
    if !(condition) {
        ({
            log_471(
                facebook_yoga_LogLevel_Fatal,
                Ptr::<i8>::from_string_literal(b"%s\n"),
                Box::new([((message).clone()).into()]),
            )
        });
        ({ fatalWithMessage_103((message).clone()) });
    }
}
pub fn assertFatalWithNode_213(
    mut node: Ptr<facebook_yoga_Node>,
    mut condition: bool,
    mut message: Ptr<i8>,
) {
    if !(condition) {
        ({
            log_472(
                (node).clone(),
                facebook_yoga_LogLevel_Fatal,
                Ptr::<i8>::from_string_literal(b"%s\n"),
                Box::new([((message).clone()).into()]),
            )
        });
        ({ fatalWithMessage_103((message).clone()) });
    }
}
pub fn assertFatalWithConfig_189(
    mut config: Ptr<facebook_yoga_Config>,
    mut condition: bool,
    mut message: Ptr<i8>,
) {
    if !(condition) {
        ({
            log_473(
                (config).clone(),
                facebook_yoga_LogLevel_Fatal,
                Ptr::<i8>::from_string_literal(b"%s\n"),
                Box::new([((message).clone()).into()]),
            )
        });
        ({ fatalWithMessage_103((message).clone()) });
    }
}
pub fn vlog_474(
    mut config: Ptr<facebook_yoga_Config>,
    mut node: Ptr<facebook_yoga_Node>,
    mut level: facebook_yoga_LogLevel,
    mut format: Ptr<i8>,
    args: Ptr<VaArg>,
) {
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(args));
    if (config).is_null() {
        ({
            ({ getDefaultLogger_183() }).call(
                Ptr::<YGConfig>::null(),
                node.reinterpret_cast::<YGNode>(),
                ({ unscopedEnum_68(level) }),
                (format).clone(),
                (*args.borrow()).clone(),
            )
        });
    } else {
        ({
            facebook_yoga_ConfigImpl::log(
                &config,
                (node).clone(),
                level,
                (format).clone(),
                (*args.borrow()).clone(),
            )
        });
    }
}
pub fn log_471(mut level: facebook_yoga_LogLevel, format: Ptr<i8>, __args: Box<[VaArg]>) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let format: Value<Ptr<i8>> = Rc::new(RefCell::new(format));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    ({
        vlog_474(
            Ptr::<facebook_yoga_Config>::null(),
            Ptr::<facebook_yoga_Node>::null(),
            level,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn log_472(
    mut node: Ptr<facebook_yoga_Node>,
    mut level: facebook_yoga_LogLevel,
    format: Ptr<i8>,
    __args: Box<[VaArg]>,
) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let format: Value<Ptr<i8>> = Rc::new(RefCell::new(format));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    ({
        let _config: Ptr<facebook_yoga_Config> = if (node).is_null() {
            Ptr::<facebook_yoga_Config>::null()
        } else {
            ({ facebook_yoga_NodeImpl::getConfig(&node) })
        };
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        vlog_474(
            _config,
            _node,
            level,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn log_473(
    mut config: Ptr<facebook_yoga_Config>,
    mut level: facebook_yoga_LogLevel,
    format: Ptr<i8>,
    __args: Box<[VaArg]>,
) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let format: Value<Ptr<i8>> = Rc::new(RefCell::new(format));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    ({
        vlog_474(
            (config).clone(),
            Ptr::<facebook_yoga_Node>::null(),
            level,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn getDefaultLogger_183()
-> FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32> {
    return FnPtr::<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>::new(
        |mut _a0: Ptr<YGConfig>,
         mut _a1: Ptr<YGNode>,
         mut level: YGLogLevel,
         mut format: Ptr<i8>,
         args: Ptr<VaArg>|
         -> i32 {
            {
                let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(args));
                'switch: {
                    match { (level as i32) } {
                        __v if __v == (YGLogLevel_YGLogLevelError as i32)
                            || __v == (YGLogLevel_YGLogLevelFatal as i32) =>
                        {
                            return {
                                let __fmt = format.to_rust_string();
                                let __n = __fmt.matches('%').count();
                                let __va: Vec<VaArg> = (0..__n)
                                    .map(|__i| (*args.borrow()).offset(__i as isize).read())
                                    .collect();
                                let __s = libcc2rs::format_c(&__fmt, &__va);
                                libcc2rs::c_stderr().with_mut(|__f| __f.write(__s.as_bytes()))
                                    as i32
                            };
                        }
                        _ => {
                            return {
                                let __fmt = format.to_rust_string();
                                let __n = __fmt.matches('%').count();
                                let __va: Vec<VaArg> = (0..__n)
                                    .map(|__i| (*args.borrow()).offset(__i as isize).read())
                                    .collect();
                                let __s = libcc2rs::format_c(&__fmt, &__va);
                                libcc2rs::c_stdout().with_mut(|__f| __f.write(__s.as_bytes()))
                                    as i32
                            };
                        }
                    }
                };
                panic!("ub: non-void function does not return a value")
            }
        },
    );
}
pub fn LayoutPassReasonToString_475(mut value: facebook_yoga_LayoutPassReason) -> Ptr<i8> {
    'switch: {
        match { value } {
            __v if __v == facebook_yoga_LayoutPassReason_kInitial => {
                return Ptr::<i8>::from_string_literal(b"initial");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kAbsLayout => {
                return Ptr::<i8>::from_string_literal(b"abs_layout");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kStretch => {
                return Ptr::<i8>::from_string_literal(b"stretch");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kMultilineStretch => {
                return Ptr::<i8>::from_string_literal(b"multiline_stretch");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kFlexLayout => {
                return Ptr::<i8>::from_string_literal(b"flex_layout");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kMeasureChild => {
                return Ptr::<i8>::from_string_literal(b"measure");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kAbsMeasureChild => {
                return Ptr::<i8>::from_string_literal(b"abs_measure");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kFlexMeasure => {
                return Ptr::<i8>::from_string_literal(b"flex_measure");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kGridLayout => {
                return Ptr::<i8>::from_string_literal(b"grid_layout");
            }
            _ => {
                return Ptr::<i8>::from_string_literal(b"unknown");
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(40)]
pub struct facebook_yoga__anonymous_namespace__Node {
    #[offset(0)]
    #[byte_size(32)]
    pub subscriber:
        FnPtr<fn(Ptr<YGNode>, facebook_yoga_Event_Type, facebook_yoga_Event_Data) -> ()>,
    #[offset(32)]
    #[byte_size(8)]
    pub next: Ptr<facebook_yoga__anonymous_namespace__Node>,
}
impl facebook_yoga__anonymous_namespace__Node {
    pub fn new(
        subscriber: Ptr<
            FnPtr<fn(Ptr<YGNode>, facebook_yoga_Event_Type, facebook_yoga_Event_Data) -> ()>,
        >,
    ) -> Self {
        Self {
            subscriber: (*subscriber.upgrade().deref()).move_from(),
            next: Ptr::<facebook_yoga__anonymous_namespace__Node>::null(),
        }
    }
    pub fn move_from(_a0: Ptr<facebook_yoga__anonymous_namespace__Node>) -> Self {
        Self {
            subscriber: (*_a0.upgrade().deref()).subscriber.move_from(),
            next: { (*_a0.upgrade().deref()).next.clone() },
        }
    }
}
impl Default for facebook_yoga__anonymous_namespace__Node {
    fn default() -> Self {
        {
            facebook_yoga__anonymous_namespace__Node {
                subscriber: FnPtr::null(),
                next: Ptr::<facebook_yoga__anonymous_namespace__Node>::null(),
            }
        }
    }
}
thread_local!(
    pub static subscribers_476: Value<Ptr<facebook_yoga__anonymous_namespace__Node>> = Rc::new(
        RefCell::new(Ptr::<facebook_yoga__anonymous_namespace__Node>::null()),
    );
);
pub fn push_477(
    mut newHead: Ptr<facebook_yoga__anonymous_namespace__Node>,
) -> Ptr<facebook_yoga__anonymous_namespace__Node> {
    let mut oldHead: Ptr<facebook_yoga__anonymous_namespace__Node> =
        Ptr::<facebook_yoga__anonymous_namespace__Node>::null();
    let mut __do_while = true;
    'loop_: while __do_while
        || (!(if ({ (*subscribers_476.with(Value::clone).borrow()).clone() }) == (oldHead).clone() {
            (*subscribers_476.with(Value::clone).borrow_mut()) = (newHead).clone();
            true
        } else {
            oldHead = (*subscribers_476.with(Value::clone).borrow()).clone();
            false
        }))
    {
        __do_while = false;
        oldHead = (*subscribers_476.with(Value::clone).borrow()).clone();
        if !((newHead).is_null()) {
            field!(newHead, next).write((oldHead).clone());
        }
    }
    return oldHead;
}
impl facebook_yoga_Event {
    pub fn reset() {
        let mut head: Ptr<facebook_yoga__anonymous_namespace__Node> =
            ({ push_477(Ptr::<facebook_yoga__anonymous_namespace__Node>::null()) });
        'loop_: while !((head).is_null()) {
            let mut current: Ptr<facebook_yoga__anonymous_namespace__Node> = (head).clone();
            head = { head.with(|__s| __s.next.clone()) };
            current.delete();
        }
    }
}
impl facebook_yoga_Event {
    pub fn subscribe(
        subscriber: Ptr<
            FnPtr<fn(Ptr<YGNode>, facebook_yoga_Event_Type, facebook_yoga_Event_Data) -> ()>,
        >,
    ) {
        ({
            push_477(Ptr::alloc(facebook_yoga__anonymous_namespace__Node::new({
                (subscriber ).reinterpret_cast::<FnPtr<fn(Ptr< YGNode  >  , facebook_yoga_Event_Type  , facebook_yoga_Event_Data  ) -> ()  >  >()
            })))
        });
    }
}
impl facebook_yoga_Event {
    fn publish(
        mut node: Ptr<YGNode>,
        mut eventType: facebook_yoga_Event_Type,
        eventData: Ptr<facebook_yoga_Event_Data>,
    ) {
        let mut subscriber: Ptr<facebook_yoga__anonymous_namespace__Node> =
            (*subscribers_476.with(Value::clone).borrow()).clone();
        'loop_: while !((subscriber).is_null()) {
            (*subscriber.upgrade().deref()).subscriber.call(
                (node).clone(),
                eventType,
                ({ (*eventData.upgrade().deref()).clone() }),
            );
            subscriber = { subscriber.with(|__s| __s.next.clone()) };
        }
    }
}
pub fn inexactEquals_478(val1: Ptr<Vec<f32>>, val2: Ptr<Vec<f32>>) -> bool {
    let mut areEqual: bool = true;
    let mut i: usize = 0_usize;
    'loop_: while (i < (4_u64 as usize)) && (areEqual) {
        areEqual = ({
            let _a: f32 = (val1.decay().offset(i as isize).read());
            let _b: f32 = (val2.decay().offset(i as isize).read());
            inexactEquals_111(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
pub fn inexactEquals_479(val1: Ptr<Vec<f32>>, val2: Ptr<Vec<f32>>) -> bool {
    let mut areEqual: bool = true;
    let mut i: usize = 0_usize;
    'loop_: while (i < (2_u64 as usize)) && (areEqual) {
        areEqual = ({
            let _a: f32 = (val1.decay().offset(i as isize).read());
            let _b: f32 = (val2.decay().offset(i as isize).read());
            inexactEquals_111(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
impl facebook_yoga_Node {
    pub fn new_2() -> Self {
        Self {
            hasNewLayout_: true,
            isReferenceBaseline_: false,
            isDirty_: true,
            alwaysFormsContainingBlock_: false,
            nodeType_: facebook_yoga_NodeType_Default,
            context_: AnyPtr::default(),
            measureFunc_:
                FnPtr::<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>::null(),
            minContentMeasureFunc_: FnPtr::<
                fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
            >::null(),
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: FnPtr::<fn(Ptr<YGNode>, f32, f32) -> f32>::null(),
            dirtiedFunc_: FnPtr::<fn(Ptr<YGNode>)>::null(),
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_usize,
            contentsChildrenCount_: 0_usize,
            owner_: Ptr::<facebook_yoga_Node>::null(),
            children_: Default::default(),
            config_: Ptr::<facebook_yoga_Config>::null(),
            processedDimensions_: Vec::from([
                ({ facebook_yoga_StyleSizeLength::undefined() }),
                ({ facebook_yoga_StyleSizeLength::undefined() }),
            ]),
        }
    }
    pub fn new_1(mut config: Ptr<facebook_yoga_Config>) -> Self {
        let __this: Value<facebook_yoga_Node> = Rc::new(RefCell::new(Self {
            hasNewLayout_: true,
            isReferenceBaseline_: false,
            isDirty_: true,
            alwaysFormsContainingBlock_: false,
            nodeType_: facebook_yoga_NodeType_Default,
            context_: AnyPtr::default(),
            measureFunc_:
                FnPtr::<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>::null(),
            minContentMeasureFunc_: FnPtr::<
                fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
            >::null(),
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: FnPtr::<fn(Ptr<YGNode>, f32, f32) -> f32>::null(),
            dirtiedFunc_: FnPtr::<fn(Ptr<YGNode>)>::null(),
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_usize,
            contentsChildrenCount_: 0_usize,
            owner_: Ptr::<facebook_yoga_Node>::null(),
            children_: Vec::new(),
            config_: (config).clone(),
            processedDimensions_: Vec::from([
                ({ facebook_yoga_StyleSizeLength::undefined() }),
                ({ facebook_yoga_StyleSizeLength::undefined() }),
            ]),
        }));
        let this: Ptr<facebook_yoga_Node> = __this.as_pointer();
        ({
            assertFatal_201(
                !((config).is_null()),
                Ptr::<i8>::from_string_literal(b"Attempting to construct Node with null config"),
            )
        });
        if ({ facebook_yoga_ConfigImpl::useWebDefaults(&config) }) {
            ({ facebook_yoga_NodeImpl::useWebDefaults(&this) });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(node: Ptr<facebook_yoga_Node>) -> Self {
        let __this: Value<facebook_yoga_Node> = Rc::new(RefCell::new(Self {
            hasNewLayout_: node.with(|__s| __s.hasNewLayout_),
            isReferenceBaseline_: node.with(|__s| __s.isReferenceBaseline_),
            isDirty_: node.with(|__s| __s.isDirty_),
            alwaysFormsContainingBlock_: node.with(|__s| __s.alwaysFormsContainingBlock_),
            nodeType_: node.with(|__s| __s.nodeType_),
            context_: node.with(|__s| __s.context_.clone()),
            measureFunc_: node.with(|__s| __s.measureFunc_.clone()),
            minContentMeasureFunc_: node.with(|__s| __s.minContentMeasureFunc_.clone()),
            minContentWidth_: node.with(|__s| __s.minContentWidth_.clone()),
            minContentHeight_: node.with(|__s| __s.minContentHeight_.clone()),
            baselineFunc_: node.with(|__s| __s.baselineFunc_.clone()),
            dirtiedFunc_: node.with(|__s| __s.dirtiedFunc_.clone()),
            style_: facebook_yoga_Style::move_from({ field_ptr!(node, style_) }),
            layout_: node.with(|__s| __s.layout_.clone()),
            lineIndex_: node.with(|__s| __s.lineIndex_),
            contentsChildrenCount_: node.with(|__s| __s.contentsChildrenCount_),
            owner_: node.with(|__s| __s.owner_.clone()),
            children_: field!(node, children_).with_mut(std::mem::take),
            config_: node.with(|__s| __s.config_.clone()),
            processedDimensions_: node.with(|__s| __s.processedDimensions_.clone()),
        }));
        let this: Ptr<facebook_yoga_Node> = __this.as_pointer();
        'loop_: for c in IterRange::new(
            field_ref!(this, children_).decay(),
            field_ref!(this, children_).decay().to_end(),
        ) {
            let mut c: Ptr<facebook_yoga_Node> = (c.read());
            ({ facebook_yoga_NodeImpl::setOwner(&c, (this).clone()) });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {}
pub trait facebook_yoga_CachedMeasurementImpl {
    fn operator_eq(&self, measurement: facebook_yoga_CachedMeasurement) -> bool;
}
impl facebook_yoga_CachedMeasurementImpl for Ptr<facebook_yoga_CachedMeasurement> {
    fn operator_eq(&self, mut measurement: facebook_yoga_CachedMeasurement) -> bool {
        let mut isEqual: bool = ((*self).with(|__s| __s.widthSizingMode)
            == measurement.widthSizingMode)
            && ((*self).with(|__s| __s.heightSizingMode) == measurement.heightSizingMode);
        if (!({ isUndefined_105((*self).with(|__s| __s.availableWidth)) }))
            || (!({ isUndefined_105(measurement.availableWidth) }))
        {
            isEqual = {
                (isEqual) && ((*self).with(|__s| __s.availableWidth) == measurement.availableWidth)
            };
        }
        if (!({ isUndefined_105((*self).with(|__s| __s.availableHeight)) }))
            || (!({ isUndefined_105(measurement.availableHeight) }))
        {
            isEqual = {
                (isEqual)
                    && ((*self).with(|__s| __s.availableHeight) == measurement.availableHeight)
            };
        }
        if (!({ isUndefined_105((*self).with(|__s| __s.computedWidth)) }))
            || (!({ isUndefined_105(measurement.computedWidth) }))
        {
            isEqual = {
                (isEqual) && ((*self).with(|__s| __s.computedWidth) == measurement.computedWidth)
            };
        }
        if (!({ isUndefined_105((*self).with(|__s| __s.computedHeight)) }))
            || (!({ isUndefined_105(measurement.computedHeight) }))
        {
            isEqual = {
                (isEqual) && ((*self).with(|__s| __s.computedHeight) == measurement.computedHeight)
            };
        }
        if (!({ isUndefined_105((*self).with(|__s| __s.ownerWidth)) }))
            || (!({ isUndefined_105(measurement.ownerWidth) }))
        {
            isEqual =
                { (isEqual) && ((*self).with(|__s| __s.ownerWidth) == measurement.ownerWidth) };
        }
        if (!({ isUndefined_105((*self).with(|__s| __s.ownerHeight)) }))
            || (!({ isUndefined_105(measurement.ownerHeight) }))
        {
            isEqual =
                { (isEqual) && ((*self).with(|__s| __s.ownerHeight) == measurement.ownerHeight) };
        }
        return isEqual;
    }
}
pub trait facebook_yoga_ConfigImpl {
    fn setUseWebDefaults(&self, mut useWebDefaults: bool) {
        unimplemented!()
    }
    fn useWebDefaults(&self) -> bool {
        unimplemented!()
    }
    fn setExperimentalFeatureEnabled(
        &self,
        mut feature: facebook_yoga_ExperimentalFeature,
        mut enabled: bool,
    ) {
        unimplemented!()
    }
    fn isExperimentalFeatureEnabled(&self, mut feature: facebook_yoga_ExperimentalFeature) -> bool {
        unimplemented!()
    }
    fn getEnabledExperiments(&self) -> Vec<bool> {
        unimplemented!()
    }
    fn setErrata(&self, mut errata: facebook_yoga_Errata) {
        unimplemented!()
    }
    fn addErrata(&self, mut errata: facebook_yoga_Errata) {
        unimplemented!()
    }
    fn removeErrata(&self, mut errata: facebook_yoga_Errata) {
        unimplemented!()
    }
    fn getErrata(&self) -> facebook_yoga_Errata {
        unimplemented!()
    }
    fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> bool {
        unimplemented!()
    }
    fn setPointScaleFactor(&self, mut pointScaleFactor: f32) {
        unimplemented!()
    }
    fn getPointScaleFactor(&self) -> f32 {
        unimplemented!()
    }
    fn setContext(&self, mut context: AnyPtr) {
        unimplemented!()
    }
    fn getContext(&self) -> AnyPtr {
        unimplemented!()
    }
    fn getVersion(&self) -> u32 {
        unimplemented!()
    }
    fn setLogger(
        &self,
        mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>,
    ) {
        unimplemented!()
    }
    fn log(
        &self,
        mut node: Ptr<facebook_yoga_Node>,
        mut logLevel: facebook_yoga_LogLevel,
        mut format: Ptr<i8>,
        args: Ptr<VaArg>,
    ) {
        unimplemented!()
    }
    fn setCloneNodeCallback(
        &self,
        mut cloneNode: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, usize) -> Ptr<YGNode>>,
    ) {
        unimplemented!()
    }
    fn cloneNode(
        &self,
        mut node: Ptr<YGNode>,
        mut owner: Ptr<YGNode>,
        mut childIndex: usize,
    ) -> Ptr<YGNode> {
        unimplemented!()
    }
}
impl facebook_yoga_ConfigImpl for Ptr<facebook_yoga_Config> {
    fn setUseWebDefaults(&self, mut useWebDefaults: bool) {
        field!((*self), useWebDefaults_).write(useWebDefaults);
    }
    fn useWebDefaults(&self) -> bool {
        return (*self).with(|__s| __s.useWebDefaults_);
    }
    fn setExperimentalFeatureEnabled(
        &self,
        mut feature: facebook_yoga_ExperimentalFeature,
        mut enabled: bool,
    ) {
        if ((({ facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(self, feature) }) as i32)
            != (enabled as i32))
        {
            match ({ (feature as usize) },) {
                (mut __tmp_0,) => {
                    field_ref!((*self), experimentalFeatures_)
                        .as_pointer()
                        .with_mut(|__v: &mut Vec<bool>| __v[__tmp_0 as usize] = enabled);
                    field_ref!((*self), experimentalFeatures_).as_pointer()
                }
            };
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn isExperimentalFeatureEnabled(&self, mut feature: facebook_yoga_ExperimentalFeature) -> bool {
        return (*(*self).upgrade().deref()).experimentalFeatures_[(feature as usize) as usize];
    }
    fn getEnabledExperiments(&self) -> Vec<bool> {
        return (*self).with(|__s| __s.experimentalFeatures_.clone());
    }
    fn setErrata(&self, mut errata: facebook_yoga_Errata) {
        if ((*self).with(|__s| __s.errata_) != errata) {
            field!((*self), errata_).write(errata);
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn addErrata(&self, mut errata: facebook_yoga_Errata) {
        if !({ facebook_yoga_ConfigImpl::hasErrata(self, errata) }) {
            ({
                let _a: Ptr<facebook_yoga_Errata> = field_ptr!((*self), errata_);
                operator_bitor_assign_54(_a, errata)
            });
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn removeErrata(&self, mut errata: facebook_yoga_Errata) {
        if ({ facebook_yoga_ConfigImpl::hasErrata(self, errata) }) {
            ({
                let _a: Ptr<facebook_yoga_Errata> = field_ptr!((*self), errata_);
                operator_bitand_assign_55(
                    _a,
                    ({
                        let _a: facebook_yoga_Errata = errata;
                        operator_bitnot_50(_a)
                    }),
                )
            });
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn getErrata(&self) -> facebook_yoga_Errata {
        return (*self).with(|__s| __s.errata_);
    }
    fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> bool {
        return (({
            let _a: facebook_yoga_Errata = (*self).with(|__s| __s.errata_);
            operator_bitand_52(_a, errata)
        }) != facebook_yoga_Errata_None);
    }
    fn setPointScaleFactor(&self, mut pointScaleFactor: f32) {
        if ((*self).with(|__s| __s.pointScaleFactor_) != pointScaleFactor) {
            field!((*self), pointScaleFactor_).write(pointScaleFactor);
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn getPointScaleFactor(&self) -> f32 {
        return (*self).with(|__s| __s.pointScaleFactor_);
    }
    fn setContext(&self, mut context: AnyPtr) {
        field!((*self), context_).write((context).clone());
    }
    fn getContext(&self) -> AnyPtr {
        return (*self).with(|__s| __s.context_.clone());
    }
    fn getVersion(&self) -> u32 {
        return (*self).with(|__s| __s.version_);
    }
    fn setLogger(
        &self,
        mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32>,
    ) {
        field!((*self), logger_).write((logger).clone());
    }
    fn log(
        &self,
        mut node: Ptr<facebook_yoga_Node>,
        mut logLevel: facebook_yoga_LogLevel,
        mut format: Ptr<i8>,
        args: Ptr<VaArg>,
    ) {
        let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(args));
        let mut logger: FnPtr<
            fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<i8>, Ptr<VaArg>) -> i32,
        > = if !(((*self).with(|__s| __s.logger_.clone())).is_null()) {
            (*self).with(|__s| __s.logger_.clone())
        } else {
            ({ getDefaultLogger_183() })
        };
        ({
            logger.call(
                (*self).reinterpret_cast::<YGConfig>(),
                node.reinterpret_cast::<YGNode>(),
                ({ unscopedEnum_68(logLevel) }),
                (format).clone(),
                (*args.borrow()).clone(),
            )
        });
    }
    fn setCloneNodeCallback(
        &self,
        mut cloneNode: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, usize) -> Ptr<YGNode>>,
    ) {
        field!((*self), cloneNodeCallback_).write((cloneNode).clone());
    }
    fn cloneNode(
        &self,
        mut node: Ptr<YGNode>,
        mut owner: Ptr<YGNode>,
        mut childIndex: usize,
    ) -> Ptr<YGNode> {
        let mut clone: Ptr<YGNode> = Ptr::<YGNode>::null();
        if !(((*self).with(|__s| __s.cloneNodeCallback_.clone())).is_null()) {
            clone = ({
                (*self).with(|__s| __s.cloneNodeCallback_.clone()).call(
                    (node).clone(),
                    (owner).clone(),
                    childIndex,
                )
            });
        }
        if (clone).is_null() {
            clone = ({ YGNodeClone_202((node).clone()) });
        }
        return clone;
    }
}
pub trait facebook_yoga_FloatOptionalImpl {
    fn unwrap(&self) -> f32;
    fn unwrapOrDefault(&self, defaultValue: f32) -> f32;
    fn isUndefined(&self) -> bool;
    fn isDefined(&self) -> bool;
}
impl facebook_yoga_FloatOptionalImpl for Ptr<facebook_yoga_FloatOptional> {
    fn unwrap(&self) -> f32 {
        return (*self).with(|__s| __s.value_);
    }
    fn unwrapOrDefault(&self, defaultValue: f32) -> f32 {
        let defaultValue: Value<f32> = Rc::new(RefCell::new(defaultValue));
        return if ({ facebook_yoga_FloatOptionalImpl::isUndefined(self) }) {
            (*defaultValue.borrow())
        } else {
            (*self).with(|__s| __s.value_)
        };
    }
    fn isUndefined(&self) -> bool {
        return ({ isUndefined_105((*self).with(|__s| __s.value_)) });
    }
    fn isDefined(&self) -> bool {
        return ({ isDefined_107((*self).with(|__s| __s.value_)) });
    }
}
pub trait facebook_yoga_GridLineImpl {
    fn isAuto(&self) -> bool;
    fn isInteger(&self) -> bool;
    fn isSpan(&self) -> bool;
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridLine>) -> bool;
}
impl facebook_yoga_GridLineImpl for Ptr<facebook_yoga_GridLine> {
    fn isAuto(&self) -> bool {
        return ((*self).with(|__s| __s.type_) == facebook_yoga_GridLineType_Auto);
    }
    fn isInteger(&self) -> bool {
        return ((*self).with(|__s| __s.type_) == facebook_yoga_GridLineType_Integer);
    }
    fn isSpan(&self) -> bool {
        return ((*self).with(|__s| __s.type_) == facebook_yoga_GridLineType_Span);
    }
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridLine>) -> bool {
        return ({ (*self).with(|__s| __s.type_) } == { other.with(|__s| __s.type_) })
            && ({ (*self).with(|__s| __s.integer) } == { other.with(|__s| __s.integer) });
    }
}
pub trait facebook_yoga_GridStyleImpl {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyle>) -> bool;
    fn move_assign(&self, _a0: Ptr<facebook_yoga_GridStyle>) -> Ptr<facebook_yoga_GridStyle>;
}
impl facebook_yoga_GridStyleImpl for Ptr<facebook_yoga_GridStyle> {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyle>) -> bool {
        return ({
            let __x = field_ref!((*self), templateColumns).as_pointer().decay();
            let __y = field_ref!(other, templateColumns).as_pointer().decay();
            __x.len() == __y.len()
                && (0..__x.len())
                    .all(|__i| CppEq::equal(&__x.offset(__i as isize), &__y.offset(__i as isize)))
        }) && (({
            let __x = field_ref!((*self), templateRows).as_pointer().decay();
            let __y = field_ref!(other, templateRows).as_pointer().decay();
            __x.len() == __y.len()
                && (0..__x.len())
                    .all(|__i| CppEq::equal(&__x.offset(__i as isize), &__y.offset(__i as isize)))
        }) && (({
            let __x = field_ref!((*self), autoColumns).as_pointer().decay();
            let __y = field_ref!(other, autoColumns).as_pointer().decay();
            __x.len() == __y.len()
                && (0..__x.len())
                    .all(|__i| CppEq::equal(&__x.offset(__i as isize), &__y.offset(__i as isize)))
        }) && (({
            let __x = field_ref!((*self), autoRows).as_pointer().decay();
            let __y = field_ref!(other, autoRows).as_pointer().decay();
            __x.len() == __y.len()
                && (0..__x.len())
                    .all(|__i| CppEq::equal(&__x.offset(__i as isize), &__y.offset(__i as isize)))
        }) && (({
            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, columnStart);
            facebook_yoga_GridLineImpl::operator_eq(&field_ptr!((*self), columnStart), _other)
        }) && (({
            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, columnEnd);
            facebook_yoga_GridLineImpl::operator_eq(&field_ptr!((*self), columnEnd), _other)
        }) && (({
            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, rowStart);
            facebook_yoga_GridLineImpl::operator_eq(&field_ptr!((*self), rowStart), _other)
        }) && ({
            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, rowEnd);
            facebook_yoga_GridLineImpl::operator_eq(&field_ptr!((*self), rowEnd), _other)
        })))))));
    }
    fn move_assign(&self, _a0: Ptr<facebook_yoga_GridStyle>) -> Ptr<facebook_yoga_GridStyle> {
        field_ref!((*self), templateColumns)
            .as_pointer()
            .write(field!(_a0, templateColumns).with_mut(std::mem::take));
        field_ref!((*self), templateRows)
            .as_pointer()
            .write(field!(_a0, templateRows).with_mut(std::mem::take));
        field_ref!((*self), autoColumns)
            .as_pointer()
            .write(field!(_a0, autoColumns).with_mut(std::mem::take));
        field_ref!((*self), autoRows)
            .as_pointer()
            .write(field!(_a0, autoRows).with_mut(std::mem::take));
        field!((*self), columnStart).write({ { (*_a0.upgrade().deref()).columnStart.clone() } });
        field!((*self), columnEnd).write({ { (*_a0.upgrade().deref()).columnEnd.clone() } });
        field!((*self), rowStart).write({ { (*_a0.upgrade().deref()).rowStart.clone() } });
        field!((*self), rowEnd).write({ { (*_a0.upgrade().deref()).rowEnd.clone() } });
        return (*self).clone();
    }
}
pub trait facebook_yoga_GridStyleStorageImpl {
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_GridStyleStorage>,
    ) -> Ptr<facebook_yoga_GridStyleStorage>;
    fn copy_assign(
        &self,
        other: Ptr<facebook_yoga_GridStyleStorage>,
    ) -> Ptr<facebook_yoga_GridStyleStorage>;
    fn get(&self) -> Ptr<facebook_yoga_GridStyle>;
    fn ensure(&self) -> Ptr<facebook_yoga_GridStyle>;
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyleStorage>) -> bool;
}
impl facebook_yoga_GridStyleStorageImpl for Ptr<facebook_yoga_GridStyleStorage> {
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_GridStyleStorage>,
    ) -> Ptr<facebook_yoga_GridStyleStorage> {
        field_ptr!((*self), grid_).write(
            field!(_a0, grid_)
                .with_mut(|__v: &mut Option<Value<facebook_yoga_GridStyle>>| __v.take()),
        );
        return (*self).clone();
    }
    fn copy_assign(
        &self,
        other: Ptr<facebook_yoga_GridStyleStorage>,
    ) -> Ptr<facebook_yoga_GridStyleStorage> {
        field_ptr!((*self), grid_).write(
            if other.with(|__s| __s.grid_.clone()).is_none() {
                None
            } else {
                Some(Rc::new(RefCell::new(libcc2rs::cpp_make!(
                    facebook_yoga_GridStyle,
                    ({
                        (*other
                            .with(|__s| __s.grid_.clone())
                            .as_pointer()
                            .upgrade()
                            .deref())
                        .clone()
                    })
                ))))
            }
            .take(),
        );
        return (*self).clone();
    }
    fn get(&self) -> Ptr<facebook_yoga_GridStyle> {
        return if (*self).with(|__s| __s.grid_.clone()).is_none() {
            ({ facebook_yoga_GridStyleStorage::defaults() })
        } else {
            ((*self).with(|__s| __s.grid_.clone()).as_pointer())
        };
    }
    fn ensure(&self) -> Ptr<facebook_yoga_GridStyle> {
        if (*self).with(|__s| __s.grid_.clone()).is_none() {
            field_ptr!((*self), grid_).write(
                Some(Rc::new(RefCell::new(libcc2rs::cpp_make!(
                    facebook_yoga_GridStyle,
                ))))
                .take(),
            );
        }
        return (*self).with(|__s| __s.grid_.clone()).as_pointer();
    }
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyleStorage>) -> bool {
        return (match (
            (*self).with(|__s| __s.grid_.clone()).as_ref(),
            other.with(|__s| __s.grid_.clone()).as_ref(),
        ) {
            (None, None) => true,
            (Some(__x), Some(__y)) => Rc::as_ptr(__x) as *const u8 == Rc::as_ptr(__y) as *const u8,
            _ => false,
        }) || ({
            let _other: Ptr<facebook_yoga_GridStyle> =
                ({ facebook_yoga_GridStyleStorageImpl::get(&other) });
            facebook_yoga_GridStyleImpl::operator_eq(
                &({ facebook_yoga_GridStyleStorageImpl::get(self) }),
                _other,
            )
        });
    }
}
pub trait facebook_yoga_GridTrackSizeImpl {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridTrackSize>) -> bool;
}
impl facebook_yoga_GridTrackSizeImpl for Ptr<facebook_yoga_GridTrackSize> {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridTrackSize>) -> bool {
        return ({
            let _rhs: Ptr<facebook_yoga_StyleSizeLength> = field_ptr!(other, minSizingFunction);
            facebook_yoga_StyleSizeLengthImpl::operator_eq(
                &field_ptr!((*self), minSizingFunction),
                _rhs,
            )
        }) && (({
            let _rhs: Ptr<facebook_yoga_StyleSizeLength> = field_ptr!(other, maxSizingFunction);
            facebook_yoga_StyleSizeLengthImpl::operator_eq(
                &field_ptr!((*self), maxSizingFunction),
                _rhs,
            )
        }) && (({ (*self).with(|__s| __s.baseSize) } == {
            other.with(|__s| __s.baseSize)
        }) && (({ (*self).with(|__s| __s.growthLimit) } == {
            other.with(|__s| __s.growthLimit)
        }) && ({
            ((*self).with(|__s| __s.infinitelyGrowable) as i32)
        } == {
            (other.with(|__s| __s.infinitelyGrowable) as i32)
        }))));
    }
}
pub trait facebook_yoga_LayoutResultsImpl {
    fn direction(&self) -> facebook_yoga_Direction;
    fn setDirection(&self, direction: facebook_yoga_Direction);
    fn hadOverflow(&self) -> bool;
    fn setHadOverflow(&self, hadOverflow: bool);
    fn dimension(&self, axis: facebook_yoga_Dimension) -> f32;
    fn setDimension(&self, axis: facebook_yoga_Dimension, dimension: f32);
    fn measuredDimension(&self, axis: facebook_yoga_Dimension) -> f32;
    fn rawDimension(&self, axis: facebook_yoga_Dimension) -> f32;
    fn setMeasuredDimension(&self, axis: facebook_yoga_Dimension, dimension: f32);
    fn setRawDimension(&self, axis: facebook_yoga_Dimension, dimension: f32);
    fn position(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> f32;
    fn setPosition(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: f32);
    fn margin(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> f32;
    fn setMargin(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: f32);
    fn border(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> f32;
    fn setBorder(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: f32);
    fn padding(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> f32;
    fn setPadding(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: f32);
    fn operator_eq(&self, mut layout: facebook_yoga_LayoutResults) -> bool {
        unimplemented!()
    }
}
impl facebook_yoga_LayoutResultsImpl for Ptr<facebook_yoga_LayoutResults> {
    fn direction(&self) -> facebook_yoga_Direction {
        return (*self).with(|__s| __s.direction_);
    }
    fn setDirection(&self, mut direction: facebook_yoga_Direction) {
        field!((*self), direction_).write(direction);
    }
    fn hadOverflow(&self) -> bool {
        return (*self).with(|__s| __s.hadOverflow_);
    }
    fn setHadOverflow(&self, mut hadOverflow: bool) {
        field!((*self), hadOverflow_).write(hadOverflow);
    }
    fn dimension(&self, mut axis: facebook_yoga_Dimension) -> f32 {
        return (field_ref!((*self), dimensions_)
            .decay()
            .offset((({ to_underlying_47(axis) }) as usize) as isize)
            .read());
    }
    fn setDimension(&self, mut axis: facebook_yoga_Dimension, mut dimension: f32) {
        field_ref!((*self), dimensions_)
            .decay()
            .offset((({ to_underlying_47(axis) }) as usize) as isize)
            .write(dimension);
    }
    fn measuredDimension(&self, mut axis: facebook_yoga_Dimension) -> f32 {
        return (field_ref!((*self), measuredDimensions_)
            .decay()
            .offset((({ to_underlying_47(axis) }) as usize) as isize)
            .read());
    }
    fn rawDimension(&self, mut axis: facebook_yoga_Dimension) -> f32 {
        return (field_ref!((*self), rawDimensions_)
            .decay()
            .offset((({ to_underlying_47(axis) }) as usize) as isize)
            .read());
    }
    fn setMeasuredDimension(&self, mut axis: facebook_yoga_Dimension, mut dimension: f32) {
        field_ref!((*self), measuredDimensions_)
            .decay()
            .offset((({ to_underlying_47(axis) }) as usize) as isize)
            .write(dimension);
    }
    fn setRawDimension(&self, mut axis: facebook_yoga_Dimension, mut dimension: f32) {
        field_ref!((*self), rawDimensions_)
            .decay()
            .offset((({ to_underlying_47(axis) }) as usize) as isize)
            .write(dimension);
    }
    fn position(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (field_ref!((*self), position_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .read());
    }
    fn setPosition(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: f32) {
        field_ref!((*self), position_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .write(dimension);
    }
    fn margin(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (field_ref!((*self), margin_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .read());
    }
    fn setMargin(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: f32) {
        field_ref!((*self), margin_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .write(dimension);
    }
    fn border(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (field_ref!((*self), border_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .read());
    }
    fn setBorder(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: f32) {
        field_ref!((*self), border_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .write(dimension);
    }
    fn padding(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> f32 {
        return (field_ref!((*self), padding_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .read());
    }
    fn setPadding(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: f32) {
        field_ref!((*self), padding_)
            .decay()
            .offset((({ to_underlying_48(physicalEdge) }) as usize) as isize)
            .write(dimension);
    }
    fn operator_eq(&self, layout: facebook_yoga_LayoutResults) -> bool {
        let layout: Value<facebook_yoga_LayoutResults> = Rc::new(RefCell::new(layout));
        let mut isEqual: bool = ((((((((((({
            let _val1: Ptr<Vec<f32>> = field_ref!((*self), position_).as_pointer();
            let _val2: Ptr<Vec<f32>> = field_ref!(layout.as_pointer(), position_).as_pointer();
            inexactEquals_478(_val1, _val2)
        }) && ({
            let _val1: Ptr<Vec<f32>> = field_ref!((*self), dimensions_).as_pointer();
            let _val2: Ptr<Vec<f32>> = field_ref!(layout.as_pointer(), dimensions_).as_pointer();
            inexactEquals_479(_val1, _val2)
        })) && ({
            let _val1: Ptr<Vec<f32>> = field_ref!((*self), margin_).as_pointer();
            let _val2: Ptr<Vec<f32>> = field_ref!(layout.as_pointer(), margin_).as_pointer();
            inexactEquals_478(_val1, _val2)
        })) && ({
            let _val1: Ptr<Vec<f32>> = field_ref!((*self), border_).as_pointer();
            let _val2: Ptr<Vec<f32>> = field_ref!(layout.as_pointer(), border_).as_pointer();
            inexactEquals_478(_val1, _val2)
        })) && ({
            let _val1: Ptr<Vec<f32>> = field_ref!((*self), padding_).as_pointer();
            let _val2: Ptr<Vec<f32>> = field_ref!(layout.as_pointer(), padding_).as_pointer();
            inexactEquals_478(_val1, _val2)
        })) && (({
            facebook_yoga_LayoutResultsImpl::direction(self)
        }) == ({
            facebook_yoga_LayoutResultsImpl::direction(&layout.as_pointer())
        }))) && ((({
            facebook_yoga_LayoutResultsImpl::hadOverflow(self)
        }) as i32)
            == (({ facebook_yoga_LayoutResultsImpl::hadOverflow(&layout.as_pointer()) })
                as i32)))
            && ((*self).with(|__s| __s.lastOwnerDirection) == {
                (*layout.borrow()).lastOwnerDirection
            }))
            && ((*self).with(|__s| __s.configVersion) == { (*layout.borrow()).configVersion }))
            && ((*self).with(|__s| __s.nextCachedMeasurementsIndex) == {
                (*layout.borrow()).nextCachedMeasurementsIndex
            }))
            && ({
                let _measurement: facebook_yoga_CachedMeasurement =
                    { (*layout.borrow()).cachedLayout.clone() };
                facebook_yoga_CachedMeasurementImpl::operator_eq(
                    &field_ptr!((*self), cachedLayout),
                    _measurement,
                )
            }))
            && ({
                let _lhs: facebook_yoga_FloatOptional =
                    (*self).with(|__s| __s.computedFlexBasis.clone());
                let _rhs: facebook_yoga_FloatOptional =
                    { (*layout.borrow()).computedFlexBasis.clone() };
                operator_eq_114(_lhs, _rhs)
            });
        let mut i: u32 = 0_u32;
        'loop_: while (i < (MaxCachedMeasurements_123.with(|rc| *rc.borrow()) as u32)) && (isEqual)
        {
            let __rhs = (isEqual)
                && ({
                    let _measurement: facebook_yoga_CachedMeasurement =
                        (*field_ref!(layout.as_pointer(), cachedMeasurements)
                            .decay()
                            .offset((i as usize) as isize)
                            .upgrade()
                            .deref())
                        .clone();
                    facebook_yoga_CachedMeasurementImpl::operator_eq(
                        &field_ref!((*self), cachedMeasurements)
                            .decay()
                            .offset((i as usize) as isize),
                        _measurement,
                    )
                });
            isEqual = __rhs;
            i.prefix_inc();
        }
        if (!({
            isUndefined_105(
                (field_ref!((*self), measuredDimensions_)
                    .decay()
                    .offset(0_usize as isize)
                    .read()),
            )
        })) || (!({
            isUndefined_105(
                (field_ref!(layout.as_pointer(), measuredDimensions_)
                    .decay()
                    .offset(0_usize as isize)
                    .read()),
            )
        })) {
            let __rhs = (isEqual)
                && ((field_ref!((*self), measuredDimensions_)
                    .decay()
                    .offset(0_usize as isize)
                    .read())
                    == (field_ref!(layout.as_pointer(), measuredDimensions_)
                        .decay()
                        .offset(0_usize as isize)
                        .read()));
            isEqual = __rhs;
        }
        if (!({
            isUndefined_105(
                (field_ref!((*self), measuredDimensions_)
                    .decay()
                    .offset(1_usize as isize)
                    .read()),
            )
        })) || (!({
            isUndefined_105(
                (field_ref!(layout.as_pointer(), measuredDimensions_)
                    .decay()
                    .offset(1_usize as isize)
                    .read()),
            )
        })) {
            let __rhs = (isEqual)
                && ((field_ref!((*self), measuredDimensions_)
                    .decay()
                    .offset(1_usize as isize)
                    .read())
                    == (field_ref!(layout.as_pointer(), measuredDimensions_)
                        .decay()
                        .offset(1_usize as isize)
                        .read()));
            isEqual = __rhs;
        }
        return isEqual;
    }
}
pub trait facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl {
    fn begin(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator;
    fn end(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator;
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl
    for Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_>
{
    fn begin(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        if (({ facebook_yoga_NodeImpl::getChildCount(&(*self).with(|__s| __s.node_.clone())) })
            > 0_usize)
        {
            let result: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new(
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::new_1(
                        { (*self).with(|__s| __s.node_.clone()) },
                        { 0_usize },
                    ),
                ));
            if (({
                facebook_yoga_StyleImpl::display(
                    &({
                        facebook_yoga_NodeImpl::style_24(
                            &({
                                facebook_yoga_NodeImpl::getChild(
                                    &(*self).with(|__s| __s.node_.clone()),
                                    0_usize,
                                )
                            }),
                        )
                    }),
                )
            }) == facebook_yoga_Display_Contents)
            {
                {
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: skipContentsNodes ( &result .as_pointer()  , )
                    });
                };
            }
            return facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::move_from({
                result.as_pointer()
            });
        } else {
            return <facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>::default();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn end(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        return <facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>::default();
    }
}
pub trait facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl {
    fn operator_deref(&self) -> Ptr<facebook_yoga_Node> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>;
    fn operator_post_inc_5(
        &self,
        _a0: i32,
    ) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator;
    fn next(&self);
    fn skipContentsNodes(&self);
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    ) -> Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>;
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl
    for Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>
{
    fn operator_inc_4(&self) -> Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> {
        ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::next(self) });
        return (*self).clone();
    }
    fn operator_post_inc_5(
        &self,
        mut _a0: i32,
    ) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
        let tmp: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                &(*self),
            )
        });
        return facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::move_from({
            tmp.as_pointer()
        });
    }
    fn next(&self) {
        if ({ ((*self).with(|__s| __s.childIndex_)).wrapping_add(1_usize) } >= {
            ({ facebook_yoga_NodeImpl::getChildCount(&(*self).with(|__s| __s.node_.clone())) })
        }) {
            if (*(*self).upgrade().deref()).backtrack_.is_empty() {
                {
                    ({
                        let _arg0 : Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator > = Rc::new(RefCell::new(<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator  >::default() ));
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: move_assign ( &(* self )   , _arg0.as_pointer() , )
                    });
                };
            } else {
                let back: Ptr<CppPair<Ptr<facebook_yoga_Node>, u64>> =
                    field_ref!((*self), backtrack_).decay();
                field!((*self), node_).write(back.with(|__s| __s.first.clone()));
                field!((*self), childIndex_).write((back.with(|__s| __s.second) as usize));
                field!((*self), backtrack_)
                    .with_mut(|__v: &mut Vec<CppPair<Ptr<facebook_yoga_Node>, u64>>| __v.remove(0));
                ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::next(self) });
            }
        } else {
            field!((*self), childIndex_).with_mut(|__v| __v.prefix_inc());
            if (({
                facebook_yoga_StyleImpl::display(
                    &({
                        facebook_yoga_NodeImpl::style_24(
                            &({
                                let _index: usize = (*self).with(|__s| __s.childIndex_);
                                facebook_yoga_NodeImpl::getChild(
                                    &(*self).with(|__s| __s.node_.clone()),
                                    _index,
                                )
                            }),
                        )
                    }),
                )
            }) == facebook_yoga_Display_Contents)
            {
                {
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: skipContentsNodes ( self , )
                    });
                };
            }
        }
    }
    fn skipContentsNodes(&self) {
        let mut currentNode: Ptr<facebook_yoga_Node> = ({
            let _index: usize = (*self).with(|__s| __s.childIndex_);
            facebook_yoga_NodeImpl::getChild(&(*self).with(|__s| __s.node_.clone()), _index)
        });
        'loop_: while (({
            facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&currentNode) }))
        }) == facebook_yoga_Display_Contents)
            && (({ facebook_yoga_NodeImpl::getChildCount(&currentNode) }) > 0_usize)
        {
            {
                let __a1 = CppPair {
                    first: (*self).with(|__s| __s.node_.clone()),
                    second: ((*self).with(|__s| __s.childIndex_) as u64),
                };
                field!((*self), backtrack_).with_mut(
                    |__v: &mut Vec<CppPair<Ptr<facebook_yoga_Node>, u64>>| __v.insert(0, __a1),
                )
            };
            field!((*self), node_).write((currentNode).clone());
            field!((*self), childIndex_).write(0_usize);
            let __rhs = ({
                facebook_yoga_NodeImpl::getChild(&currentNode, (*self).with(|__s| __s.childIndex_))
            });
            currentNode = __rhs;
        }
        if (({
            facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&currentNode) }))
        }) == facebook_yoga_Display_Contents)
        {
            ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::next(self) });
        }
    }
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    ) -> Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> {
        field!((*self), node_).write({ { (*_a0.upgrade().deref()).node_.clone() } });
        field!((*self), childIndex_).write({ { (*_a0.upgrade().deref()).childIndex_ } });
        field_ref!((*self), backtrack_)
            .as_pointer()
            .write(field!(_a0, backtrack_).with_mut(std::mem::take));
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<facebook_yoga_Node> {
        return ({
            let _index: usize = (*self).with(|__s| __s.childIndex_);
            facebook_yoga_NodeImpl::getChild(&(*self).with(|__s| __s.node_.clone()), _index)
        });
    }
}
pub trait facebook_yoga_NodeImpl {
    fn getContext(&self) -> AnyPtr;
    fn alwaysFormsContainingBlock(&self) -> bool;
    fn getHasNewLayout(&self) -> bool;
    fn getNodeType(&self) -> facebook_yoga_NodeType;
    fn hasMeasureFunc(&self) -> bool;
    fn measure(
        &self,
        mut availableWidth: f32,
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: f32,
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        unimplemented!()
    }
    fn hasMinContentMeasureFunc(&self) -> bool;
    fn measureMinContent(
        &self,
        mut availableWidth: f32,
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: f32,
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        unimplemented!()
    }
    fn getMinContentWidth(&self) -> facebook_yoga_FloatOptional;
    fn getMinContentHeight(&self) -> facebook_yoga_FloatOptional;
    fn hasBaselineFunc(&self) -> bool;
    fn baseline(&self, mut width: f32, mut height: f32) -> f32 {
        unimplemented!()
    }
    fn dimensionWithMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut widthSize: f32,
    ) -> f32 {
        unimplemented!()
    }
    fn isLayoutDimensionDefined(&self, mut axis: facebook_yoga_FlexDirection) -> bool {
        unimplemented!()
    }
    fn hasDefiniteLength(&self, dimension: facebook_yoga_Dimension, ownerSize: f32) -> bool;
    fn hasErrata(&self, errata: facebook_yoga_Errata) -> bool;
    fn hasContentsChildren(&self) -> bool;
    fn getDirtiedFunc(&self) -> FnPtr<fn(Ptr<YGNode>)>;
    fn style_24(&self) -> Ptr<facebook_yoga_Style>;
    fn style_25(&self) -> Ptr<facebook_yoga_Style>;
    fn getLayout_26(&self) -> Ptr<facebook_yoga_LayoutResults>;
    fn getLayout_27(&self) -> Ptr<facebook_yoga_LayoutResults>;
    fn getLineIndex(&self) -> usize;
    fn isReferenceBaseline(&self) -> bool;
    fn getOwner(&self) -> Ptr<facebook_yoga_Node>;
    fn getChildren(&self) -> Ptr<Vec<Ptr<facebook_yoga_Node>>>;
    fn getChild(&self, index: usize) -> Ptr<facebook_yoga_Node>;
    fn getChildCount(&self) -> usize;
    fn getLayoutChildren(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node_;
    fn getLayoutChildCount(&self) -> usize;
    fn getConfig(&self) -> Ptr<facebook_yoga_Config>;
    fn isDirty(&self) -> bool;
    fn getProcessedDimension(
        &self,
        dimension: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength;
    fn getResolvedDimension(
        &self,
        direction: facebook_yoga_Direction,
        dimension: facebook_yoga_Dimension,
        referenceLength: f32,
        ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional;
    fn setContext(&self, context: AnyPtr);
    fn setAlwaysFormsContainingBlock(&self, alwaysFormsContainingBlock: bool);
    fn setHasNewLayout(&self, hasNewLayout: bool);
    fn setNodeType(&self, nodeType: facebook_yoga_NodeType);
    fn setMeasureFunc(
        &self,
        mut measureFunc: FnPtr<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
    ) {
        unimplemented!()
    }
    fn setMinContentMeasureFunc(
        &self,
        minContentMeasureFunc: FnPtr<
            fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
        >,
    );
    fn setMinContentWidth(&self, minContentWidth: facebook_yoga_FloatOptional);
    fn setMinContentHeight(&self, minContentHeight: facebook_yoga_FloatOptional);
    fn setBaselineFunc(&self, baseLineFunc: FnPtr<fn(Ptr<YGNode>, f32, f32) -> f32>);
    fn setDirtiedFunc(&self, dirtiedFunc: FnPtr<fn(Ptr<YGNode>)>);
    fn setStyle(&self, style: Ptr<facebook_yoga_Style>);
    fn setLayout(&self, layout: Ptr<facebook_yoga_LayoutResults>);
    fn setLineIndex(&self, lineIndex: usize);
    fn setIsReferenceBaseline(&self, isReferenceBaseline: bool);
    fn setOwner(&self, owner: Ptr<facebook_yoga_Node>);
    fn setConfig(&self, mut config: Ptr<facebook_yoga_Config>) {
        unimplemented!()
    }
    fn setDirty(&self, mut isDirty: bool) {
        unimplemented!()
    }
    fn setChildren(&self, children: Ptr<Vec<Ptr<facebook_yoga_Node>>>) {
        unimplemented!()
    }
    fn setLayoutLastOwnerDirection(&self, mut direction: facebook_yoga_Direction) {
        unimplemented!()
    }
    fn setLayoutComputedFlexBasis(&self, mut computedFlexBasis: facebook_yoga_FloatOptional) {
        unimplemented!()
    }
    fn setLayoutComputedFlexBasisGeneration(&self, mut computedFlexBasisGeneration: u32) {
        unimplemented!()
    }
    fn setLayoutMeasuredDimension(
        &self,
        mut measuredDimension: f32,
        mut dimension: facebook_yoga_Dimension,
    ) {
        unimplemented!()
    }
    fn setLayoutHadOverflow(&self, mut hadOverflow: bool) {
        unimplemented!()
    }
    fn setLayoutDimension(&self, mut lengthValue: f32, mut dimension: facebook_yoga_Dimension) {
        unimplemented!()
    }
    fn setLayoutDirection(&self, mut direction: facebook_yoga_Direction) {
        unimplemented!()
    }
    fn setLayoutMargin(&self, mut margin: f32, mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setLayoutBorder(&self, mut border: f32, mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setLayoutPadding(&self, mut padding: f32, mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setLayoutPosition(&self, mut position: f32, mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setPosition(
        &self,
        mut direction: facebook_yoga_Direction,
        mut ownerWidth: f32,
        mut ownerHeight: f32,
    ) {
        unimplemented!()
    }
    fn processFlexBasis(&self) -> facebook_yoga_StyleSizeLength {
        unimplemented!()
    }
    fn resolveFlexBasis(
        &self,
        mut direction: facebook_yoga_Direction,
        mut flexDirection: facebook_yoga_FlexDirection,
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        unimplemented!()
    }
    fn processDimensions(&self) {
        unimplemented!()
    }
    fn resolveDirection(
        &self,
        mut ownerDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_Direction {
        unimplemented!()
    }
    fn clearChildren(&self) {
        unimplemented!()
    }
    fn replaceChild_75(
        &self,
        mut oldChild: Ptr<facebook_yoga_Node>,
        mut newChild: Ptr<facebook_yoga_Node>,
    ) {
        unimplemented!()
    }
    fn replaceChild_76(&self, mut child: Ptr<facebook_yoga_Node>, mut index: usize) {
        unimplemented!()
    }
    fn insertChild(&self, mut child: Ptr<facebook_yoga_Node>, mut index: usize) {
        unimplemented!()
    }
    fn removeChild_78(&self, mut child: Ptr<facebook_yoga_Node>) -> bool {
        unimplemented!()
    }
    fn removeChild_79(&self, mut index: usize) {
        unimplemented!()
    }
    fn cloneChildrenIfNeeded(&self) {
        unimplemented!()
    }
    fn cloneContentsChildrenIfNeeded(&self) {
        unimplemented!()
    }
    fn markDirtyAndPropagate(&self) {
        unimplemented!()
    }
    fn resolveFlexGrow(&self) -> f32 {
        unimplemented!()
    }
    fn resolveFlexShrink(&self) -> f32 {
        unimplemented!()
    }
    fn isNodeFlexible(&self) -> bool {
        unimplemented!()
    }
    fn reset(&self) {
        unimplemented!()
    }
    fn relativePosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: f32,
    ) -> f32 {
        unimplemented!()
    }
    fn useWebDefaults(&self);
}
impl facebook_yoga_NodeImpl for Ptr<facebook_yoga_Node> {
    fn getContext(&self) -> AnyPtr {
        return (*self).with(|__s| __s.context_.clone());
    }
    fn alwaysFormsContainingBlock(&self) -> bool {
        return (*self).with(|__s| __s.alwaysFormsContainingBlock_);
    }
    fn getHasNewLayout(&self) -> bool {
        return (*self).with(|__s| __s.hasNewLayout_);
    }
    fn getNodeType(&self) -> facebook_yoga_NodeType {
        return (*self).with(|__s| __s.nodeType_);
    }
    fn hasMeasureFunc(&self) -> bool {
        return !(((*self).with(|__s| __s.measureFunc_.clone())).is_null());
    }
    fn hasMinContentMeasureFunc(&self) -> bool {
        return !(((*self).with(|__s| __s.minContentMeasureFunc_.clone())).is_null());
    }
    fn getMinContentWidth(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.minContentWidth_.clone());
    }
    fn getMinContentHeight(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.minContentHeight_.clone());
    }
    fn hasBaselineFunc(&self) -> bool {
        return !(((*self).with(|__s| __s.baselineFunc_.clone())).is_null());
    }
    fn hasDefiniteLength(
        &self,
        mut dimension: facebook_yoga_Dimension,
        mut ownerSize: f32,
    ) -> bool {
        let usedValue: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
            ({
                facebook_yoga_StyleSizeLengthImpl::resolve(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_NodeImpl::getProcessedDimension(self, dimension) }),
                    ))
                    .as_pointer(),
                    ownerSize,
                )
            }),
        ));
        return ({ facebook_yoga_FloatOptionalImpl::isDefined(&usedValue.as_pointer()) })
            && (({ facebook_yoga_FloatOptionalImpl::unwrap(&usedValue.as_pointer()) }) >= 0_f32);
    }
    fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> bool {
        return ({
            facebook_yoga_ConfigImpl::hasErrata(&(*self).with(|__s| __s.config_.clone()), errata)
        });
    }
    fn hasContentsChildren(&self) -> bool {
        return ((*self).with(|__s| __s.contentsChildrenCount_) != 0_usize);
    }
    fn getDirtiedFunc(&self) -> FnPtr<fn(Ptr<YGNode>)> {
        return (*self).with(|__s| __s.dirtiedFunc_.clone());
    }
    fn style_24(&self) -> Ptr<facebook_yoga_Style> {
        return field_ptr!((*self), style_);
    }
    fn style_25(&self) -> Ptr<facebook_yoga_Style> {
        return field_ptr!((*self), style_);
    }
    fn getLayout_26(&self) -> Ptr<facebook_yoga_LayoutResults> {
        return field_ptr!((*self), layout_);
    }
    fn getLayout_27(&self) -> Ptr<facebook_yoga_LayoutResults> {
        return field_ptr!((*self), layout_);
    }
    fn getLineIndex(&self) -> usize {
        return (*self).with(|__s| __s.lineIndex_);
    }
    fn isReferenceBaseline(&self) -> bool {
        return (*self).with(|__s| __s.isReferenceBaseline_);
    }
    fn getOwner(&self) -> Ptr<facebook_yoga_Node> {
        return (*self).with(|__s| __s.owner_.clone());
    }
    fn getChildren(&self) -> Ptr<Vec<Ptr<facebook_yoga_Node>>> {
        return field_ref!((*self), children_).as_pointer();
    }
    fn getChild(&self, mut index: usize) -> Ptr<facebook_yoga_Node> {
        return ({
            let __idx = index;
            if __idx as usize
                >= (*field_ref!((*self), children_)
                    .as_pointer()
                    .upgrade()
                    .deref())
                .len()
            {
                panic!("out of bounds access")
            } else {
                field_ref!((*self), children_)
                    .as_pointer()
                    .decay()
                    .offset(__idx as isize)
            }
        }
        .read());
    }
    fn getChildCount(&self) -> usize {
        return (*(*self).upgrade().deref()).children_.len();
    }
    fn getLayoutChildren(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
        return facebook_yoga_LayoutableChildren_facebook_yoga_Node_::new({ (*self).clone() });
    }
    fn getLayoutChildCount(&self) -> usize {
        if ((*self).with(|__s| __s.contentsChildrenCount_) == 0_usize) {
            return (*(*self).upgrade().deref()).children_.len();
        } else {
            let mut count: usize = 0_usize;
            let iter: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new(
                    ({
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::begin(
                            &Rc::new(RefCell::new(
                                ({ facebook_yoga_NodeImpl::getLayoutChildren(self) }),
                            ))
                            .as_pointer(),
                        )
                    }),
                ));
            'loop_: while ({
                let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    iter.as_pointer();
                let _b: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    Rc::new(RefCell::new(
                        ({
                            facebook_yoga_LayoutableChildren_facebook_yoga_Node_Impl::end(
                                &Rc::new(RefCell::new(
                                    ({ facebook_yoga_NodeImpl::getLayoutChildren(self) }),
                                ))
                                .as_pointer(),
                            )
                        }),
                    ));
                operator_ne_78(_a, _b.as_pointer())
            }) {
                count.postfix_inc();
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iter .as_pointer()  , 0  , )
                });
            }
            return count;
        }
        panic!("ub: non-void function does not return a value")
    }
    fn getConfig(&self) -> Ptr<facebook_yoga_Config> {
        return (*self).with(|__s| __s.config_.clone());
    }
    fn isDirty(&self) -> bool {
        return (*self).with(|__s| __s.isDirty_);
    }
    fn getProcessedDimension(
        &self,
        mut dimension: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (*field_ref!((*self), processedDimensions_)
            .decay()
            .offset((dimension as usize) as isize)
            .upgrade()
            .deref())
        .clone();
    }
    fn getResolvedDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        let mut value: facebook_yoga_FloatOptional = ({
            facebook_yoga_StyleSizeLengthImpl::resolve(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_NodeImpl::getProcessedDimension(self, dimension) }),
                ))
                .as_pointer(),
                referenceLength,
            )
        });
        if (({ facebook_yoga_StyleImpl::boxSizing(&field_ptr!((*self), style_)) })
            == facebook_yoga_BoxSizing_BorderBox)
        {
            return value;
        }
        let dimensionPaddingAndBorder: Value<facebook_yoga_FloatOptional> =
            Rc::new(RefCell::new(facebook_yoga_FloatOptional::new_1({
                ({
                    facebook_yoga_StyleImpl::computePaddingAndBorderForDimension(
                        &field_ptr!((*self), style_),
                        direction,
                        dimension,
                        ownerWidth,
                    )
                })
            })));
        return ({
            let _lhs: facebook_yoga_FloatOptional = value;
            operator_add_117(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    fn setContext(&self, mut context: AnyPtr) {
        field!((*self), context_).write((context).clone());
    }
    fn setAlwaysFormsContainingBlock(&self, mut alwaysFormsContainingBlock: bool) {
        field!((*self), alwaysFormsContainingBlock_).write(alwaysFormsContainingBlock);
    }
    fn setHasNewLayout(&self, mut hasNewLayout: bool) {
        field!((*self), hasNewLayout_).write(hasNewLayout);
    }
    fn setNodeType(&self, mut nodeType: facebook_yoga_NodeType) {
        field!((*self), nodeType_).write(nodeType);
    }
    fn setMinContentMeasureFunc(
        &self,
        mut minContentMeasureFunc: FnPtr<
            fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize,
        >,
    ) {
        field!((*self), minContentMeasureFunc_).write((minContentMeasureFunc).clone());
    }
    fn setMinContentWidth(&self, mut minContentWidth: facebook_yoga_FloatOptional) {
        field!((*self), minContentWidth_).write((minContentWidth).clone());
    }
    fn setMinContentHeight(&self, mut minContentHeight: facebook_yoga_FloatOptional) {
        field!((*self), minContentHeight_).write((minContentHeight).clone());
    }
    fn setBaselineFunc(&self, mut baseLineFunc: FnPtr<fn(Ptr<YGNode>, f32, f32) -> f32>) {
        field!((*self), baselineFunc_).write((baseLineFunc).clone());
    }
    fn setDirtiedFunc(&self, mut dirtiedFunc: FnPtr<fn(Ptr<YGNode>)>) {
        field!((*self), dirtiedFunc_).write((dirtiedFunc).clone());
    }
    fn setStyle(&self, style: Ptr<facebook_yoga_Style>) {
        ({
            let _arg0: Ptr<facebook_yoga_Style> = (style).clone();
            facebook_yoga_StyleImpl::copy_assign(&field_ptr!((*self), style_), _arg0)
        });
    }
    fn setLayout(&self, layout: Ptr<facebook_yoga_LayoutResults>) {
        field!((*self), layout_).write({ (*layout.upgrade().deref()).clone() });
    }
    fn setLineIndex(&self, mut lineIndex: usize) {
        field!((*self), lineIndex_).write(lineIndex);
    }
    fn setIsReferenceBaseline(&self, mut isReferenceBaseline: bool) {
        field!((*self), isReferenceBaseline_).write(isReferenceBaseline);
    }
    fn setOwner(&self, mut owner: Ptr<facebook_yoga_Node>) {
        field!((*self), owner_).write((owner).clone());
    }
    fn useWebDefaults(&self) {
        ({
            facebook_yoga_StyleImpl::setFlexDirection(
                &field_ptr!((*self), style_),
                facebook_yoga_FlexDirection_Row,
            )
        });
        ({
            facebook_yoga_StyleImpl::setAlignContent(
                &field_ptr!((*self), style_),
                facebook_yoga_Align_Stretch,
            )
        });
    }
    fn measure(
        &self,
        mut availableWidth: f32,
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: f32,
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let mut size: YGSize = ({
            let _arg0: Ptr<YGNode> = (*self).reinterpret_cast::<YGNode>();
            let _arg2: YGMeasureMode = ({ unscopedEnum_95(widthMode) });
            let _arg4: YGMeasureMode = ({ unscopedEnum_95(heightMode) });
            (*self).with(|__s| __s.measureFunc_.clone()).call(
                _arg0,
                availableWidth,
                _arg2,
                availableHeight,
                _arg4,
            )
        });
        if ((({ isUndefined_105(size.height) }) || (size.height < 0_f32))
            || ({ isUndefined_105(size.width) }))
            || (size.width < 0_f32)
        {
            ({
                log_472 ( ((* self ) ).clone() , facebook_yoga_LogLevel_Warn  , Ptr::<i8 >::from_string_literal(b"Measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" )  , Box::new ( [ ( ( size . width  as f64 ) ) .into() , ( ( size . height  as f64 ) ) .into() , ] ) )
            });
            return YGSize {
                width: ({ maxOrDefined_110(0_f32, size.width) }),
                height: ({ maxOrDefined_110(0_f32, size.height) }),
            };
        }
        return size;
    }
    fn measureMinContent(
        &self,
        mut availableWidth: f32,
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: f32,
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let mut size: YGSize = ({
            let _arg0: Ptr<YGNode> = (*self).reinterpret_cast::<YGNode>();
            let _arg2: YGMeasureMode = ({ unscopedEnum_95(widthMode) });
            let _arg4: YGMeasureMode = ({ unscopedEnum_95(heightMode) });
            (*self).with(|__s| __s.minContentMeasureFunc_.clone()).call(
                _arg0,
                availableWidth,
                _arg2,
                availableHeight,
                _arg4,
            )
        });
        if ((({ isUndefined_105(size.height) }) || (size.height < 0_f32))
            || ({ isUndefined_105(size.width) }))
            || (size.width < 0_f32)
        {
            ({
                log_472 ( ((* self ) ).clone() , facebook_yoga_LogLevel_Warn  , Ptr::<i8 >::from_string_literal(b"Min-content measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" )  , Box::new ( [ ( ( size . width  as f64 ) ) .into() , ( ( size . height  as f64 ) ) .into() , ] ) )
            });
            let __rhs = YGSize {
                width: ({ maxOrDefined_110(0_f32, size.width) }),
                height: ({ maxOrDefined_110(0_f32, size.height) }),
            };
            size = __rhs;
        }
        return size;
    }
    fn baseline(&self, mut width: f32, mut height: f32) -> f32 {
        return ({
            let _arg0: Ptr<YGNode> = (*self).reinterpret_cast::<YGNode>();
            (*self)
                .with(|__s| __s.baselineFunc_.clone())
                .call(_arg0, width, height)
        });
    }
    fn dimensionWithMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut widthSize: f32,
    ) -> f32 {
        return (({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(self) }),
                ({ dimension_136(axis) }),
            )
        }) + ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &field_ptr!((*self), style_),
                axis,
                widthSize,
            )
        }));
    }
    fn isLayoutDimensionDefined(&self, mut axis: facebook_yoga_FlexDirection) -> bool {
        let mut value: f32 = ({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(self) }),
                ({ dimension_136(axis) }),
            )
        });
        return ({ isDefined_107(value) }) && (value >= 0_f32);
    }
    fn setMeasureFunc(
        &self,
        mut measureFunc: FnPtr<fn(Ptr<YGNode>, f32, YGMeasureMode, f32, YGMeasureMode) -> YGSize>,
    ) {
        if (measureFunc).is_null() {
            ({ facebook_yoga_NodeImpl::setNodeType(self, facebook_yoga_NodeType_Default) });
        } else {
            ({
                let _node: Ptr<facebook_yoga_Node> = (*self).clone();
                let _condition: bool = (*(*self).upgrade().deref()).children_.is_empty();
                assertFatalWithNode_213 ( _node , _condition , Ptr::<i8 >::from_string_literal(b"Cannot set measure function: Nodes with measure functions cannot have children." )  , )
            });
            ({ facebook_yoga_NodeImpl::setNodeType(self, facebook_yoga_NodeType_Text) });
        }
        field!((*self), measureFunc_).write((measureFunc).clone());
    }
    fn replaceChild_76(&self, mut child: Ptr<facebook_yoga_Node>, mut index: usize) {
        let mut previousChild: Ptr<facebook_yoga_Node> = (field_ref!((*self), children_)
            .decay()
            .offset(index as isize)
            .read());
        if (({
            facebook_yoga_StyleImpl::display(
                &({ facebook_yoga_NodeImpl::style_24(&previousChild) }),
            )
        }) == facebook_yoga_Display_Contents)
            && (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) }))
            }) != facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_dec());
        } else if (({
            facebook_yoga_StyleImpl::display(
                &({ facebook_yoga_NodeImpl::style_24(&previousChild) }),
            )
        }) != facebook_yoga_Display_Contents)
            && (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) }))
            }) == facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_inc());
        }
        field_ref!((*self), children_)
            .decay()
            .offset(index as isize)
            .write((child).clone());
    }
    fn replaceChild_75(
        &self,
        mut oldChild: Ptr<facebook_yoga_Node>,
        mut newChild: Ptr<facebook_yoga_Node>,
    ) {
        if (({
            facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&oldChild) }))
        }) == facebook_yoga_Display_Contents)
            && (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&newChild) }))
            }) != facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_dec());
        } else if (({
            facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&oldChild) }))
        }) != facebook_yoga_Display_Contents)
            && (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&newChild) }))
            }) == facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_inc());
        }
        {
            let mut __it = field_ref!((*self), children_).decay();
            let __last = field_ref!((*self), children_).decay().to_end();
            let __old = (oldChild).clone();
            let __new = (newChild).clone();
            while __it != __last {
                if __it.iter_deref().read() == __old {
                    __it.iter_deref().write(__new.clone());
                }
                __it.iter_inc();
            }
        };
    }
    fn insertChild(&self, mut child: Ptr<facebook_yoga_Node>, mut index: usize) {
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
            == facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_inc());
        }
        {
            let __pos = {
                let __it: Ptr<Ptr<facebook_yoga_Node>> = field_ref!((*self), children_)
                    .decay()
                    .iter_add((index as isize))
                    .into();
                __it
            }
            .get_offset();
            field_ref!((*self), children_).as_pointer().with_mut(
                |__v: &mut Vec<Ptr<facebook_yoga_Node>>| __v.insert(__pos, (child).clone()),
            );
            field_ref!((*self), children_)
                .as_pointer()
                .decay()
                .offset(__pos as isize)
        };
    }
    fn setConfig(&self, mut config: Ptr<facebook_yoga_Config>) {
        ({
            assertFatal_201(
                !((config).is_null()),
                Ptr::<i8>::from_string_literal(b"Attempting to set a null config on a Node"),
            )
        });
        ({
            let _config: Ptr<facebook_yoga_Config> = (config).clone();
            let _condition: bool =
                ({ (({ facebook_yoga_ConfigImpl::useWebDefaults(&config) }) as i32) } == {
                    (({
                        facebook_yoga_ConfigImpl::useWebDefaults(
                            &(*self).with(|__s| __s.config_.clone()),
                        )
                    }) as i32)
                });
            assertFatalWithConfig_189(
                _config,
                _condition,
                Ptr::<i8>::from_string_literal(
                    b"UseWebDefaults may not be changed after constructing a Node",
                ),
            )
        });
        if ({
            let _oldConfig: Ptr<facebook_yoga_Config> = (*self).with(|__s| __s.config_.clone());
            let _newConfig: Ptr<facebook_yoga_Config> = (config).clone();
            configUpdateInvalidatesLayout_470(_oldConfig, _newConfig)
        }) {
            ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(self) });
            field!(field!((*self), layout_), configVersion).write(0_u32);
        } else {
            field!(field!((*self), layout_), configVersion)
                .write(({ facebook_yoga_ConfigImpl::getVersion(&config) }));
        }
        field!((*self), config_).write((config).clone());
    }
    fn setDirty(&self, mut isDirty: bool) {
        if ((isDirty as i32) == ((*self).with(|__s| __s.isDirty_) as i32)) {
            return;
        }
        field!((*self), isDirty_).write(isDirty);
        if (isDirty) && (!(((*self).with(|__s| __s.dirtiedFunc_.clone())).is_null())) {
            ({
                let _arg0: Ptr<YGNode> = (*self).reinterpret_cast::<YGNode>();
                (*self).with(|__s| __s.dirtiedFunc_.clone()).call(_arg0)
            });
        }
    }
    fn setChildren(&self, children: Ptr<Vec<Ptr<facebook_yoga_Node>>>) {
        field_ref!((*self), children_)
            .as_pointer()
            .write(({ (*children.upgrade().deref()).clone() }));
        field!((*self), contentsChildrenCount_).write(0_usize);
        'loop_: for child in IterRange::new(children.decay(), children.decay().to_end()) {
            let child: Ptr<Ptr<facebook_yoga_Node>> = (child).clone();
            if (({
                facebook_yoga_StyleImpl::display(
                    &({ facebook_yoga_NodeImpl::style_24(&(child.read())) }),
                )
            }) == facebook_yoga_Display_Contents)
            {
                field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_inc());
            }
        }
    }
    fn removeChild_78(&self, mut child: Ptr<facebook_yoga_Node>) -> bool {
        let p: Value<Ptr<Ptr<facebook_yoga_Node>>> = Rc::new(RefCell::new({
            let mut __it = field_ref!((*self), children_).decay();
            let __last = field_ref!((*self), children_).decay().to_end();
            let __value = (child).clone();
            while __it != __last && __it.iter_deref().read() != __value {
                __it.iter_inc();
            }
            __it
        }));
        if !((*p.borrow()) == field_ref!((*self), children_).decay().to_end()) {
            if (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) }))
            }) == facebook_yoga_Display_Contents)
            {
                field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_dec());
            }
            {
                let __pos = {
                    let __it: Ptr<Ptr<facebook_yoga_Node>> = ({ (*p.borrow()).clone() }).into();
                    __it
                }
                .get_offset();
                field_ref!((*self), children_).as_pointer().with_mut(
                    |__v: &mut Vec<Ptr<facebook_yoga_Node>>| {
                        __v.remove(__pos);
                    },
                );
                field_ref!((*self), children_)
                    .as_pointer()
                    .decay()
                    .offset(__pos as isize)
            };
            return true;
        }
        return false;
    }
    fn removeChild_79(&self, mut index: usize) {
        if (({
            facebook_yoga_StyleImpl::display(
                &({
                    facebook_yoga_NodeImpl::style_24(
                        &(field_ref!((*self), children_)
                            .decay()
                            .offset(index as isize)
                            .read()),
                    )
                }),
            )
        }) == facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_dec());
        }
        {
            let __pos = {
                let __it: Ptr<Ptr<facebook_yoga_Node>> = field_ref!((*self), children_)
                    .decay()
                    .iter_add((index as isize))
                    .into();
                __it
            }
            .get_offset();
            field_ref!((*self), children_).as_pointer().with_mut(
                |__v: &mut Vec<Ptr<facebook_yoga_Node>>| {
                    __v.remove(__pos);
                },
            );
            field_ref!((*self), children_)
                .as_pointer()
                .decay()
                .offset(__pos as isize)
        };
    }
    fn setLayoutDirection(&self, mut direction: facebook_yoga_Direction) {
        ({
            facebook_yoga_LayoutResultsImpl::setDirection(&field_ptr!((*self), layout_), direction)
        });
    }
    fn setLayoutMargin(&self, mut margin: f32, mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setMargin(&field_ptr!((*self), layout_), edge, margin)
        });
    }
    fn setLayoutBorder(&self, mut border: f32, mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setBorder(&field_ptr!((*self), layout_), edge, border)
        });
    }
    fn setLayoutPadding(&self, mut padding: f32, mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setPadding(
                &field_ptr!((*self), layout_),
                edge,
                padding,
            )
        });
    }
    fn setLayoutLastOwnerDirection(&self, mut direction: facebook_yoga_Direction) {
        field!(field!((*self), layout_), lastOwnerDirection).write(direction);
    }
    fn setLayoutComputedFlexBasis(&self, mut computedFlexBasis: facebook_yoga_FloatOptional) {
        field!(field!((*self), layout_), computedFlexBasis).write((computedFlexBasis).clone());
    }
    fn setLayoutPosition(&self, mut position: f32, mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setPosition(
                &field_ptr!((*self), layout_),
                edge,
                position,
            )
        });
    }
    fn setLayoutComputedFlexBasisGeneration(&self, mut computedFlexBasisGeneration: u32) {
        field!(field!((*self), layout_), computedFlexBasisGeneration)
            .write(computedFlexBasisGeneration);
    }
    fn setLayoutMeasuredDimension(
        &self,
        mut measuredDimension: f32,
        mut dimension: facebook_yoga_Dimension,
    ) {
        ({
            facebook_yoga_LayoutResultsImpl::setMeasuredDimension(
                &field_ptr!((*self), layout_),
                dimension,
                measuredDimension,
            )
        });
    }
    fn setLayoutHadOverflow(&self, mut hadOverflow: bool) {
        ({
            facebook_yoga_LayoutResultsImpl::setHadOverflow(
                &field_ptr!((*self), layout_),
                hadOverflow,
            )
        });
    }
    fn setLayoutDimension(&self, mut lengthValue: f32, mut dimension: facebook_yoga_Dimension) {
        ({
            facebook_yoga_LayoutResultsImpl::setDimension(
                &field_ptr!((*self), layout_),
                dimension,
                lengthValue,
            )
        });
        ({
            facebook_yoga_LayoutResultsImpl::setRawDimension(
                &field_ptr!((*self), layout_),
                dimension,
                lengthValue,
            )
        });
    }
    fn relativePosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: f32,
    ) -> f32 {
        if (({ facebook_yoga_StyleImpl::positionType(&field_ptr!((*self), style_)) })
            == facebook_yoga_PositionType_Static)
        {
            return 0_f32;
        }
        if ({
            facebook_yoga_StyleImpl::isInlineStartPositionDefined(
                &field_ptr!((*self), style_),
                axis,
                direction,
            )
        }) && (!({
            facebook_yoga_StyleImpl::isInlineStartPositionAuto(
                &field_ptr!((*self), style_),
                axis,
                direction,
            )
        })) {
            return ({
                facebook_yoga_StyleImpl::computeInlineStartPosition(
                    &field_ptr!((*self), style_),
                    axis,
                    direction,
                    axisSize,
                )
            });
        }
        return ((-1_i32 as f32)
            * ({
                facebook_yoga_StyleImpl::computeInlineEndPosition(
                    &field_ptr!((*self), style_),
                    axis,
                    direction,
                    axisSize,
                )
            }));
    }
    fn setPosition(
        &self,
        mut direction: facebook_yoga_Direction,
        mut ownerWidth: f32,
        mut ownerHeight: f32,
    ) {
        let mut directionRespectingRoot: facebook_yoga_Direction =
            if !(((*self).with(|__s| __s.owner_.clone())).is_null()) {
                direction
            } else {
                facebook_yoga_Direction_LTR
            };
        let mut mainAxis: facebook_yoga_FlexDirection = ({
            resolveDirection_130(
                ({ facebook_yoga_StyleImpl::flexDirection(&field_ptr!((*self), style_)) }),
                directionRespectingRoot,
            )
        });
        let mut crossAxis: facebook_yoga_FlexDirection =
            ({ resolveCrossDirection_131(mainAxis, directionRespectingRoot) });
        let mut relativePositionMain: f32 = ({
            let _axis: facebook_yoga_FlexDirection = mainAxis;
            let _axisSize: f32 = if ({ isRow_128(mainAxis) }) {
                ownerWidth
            } else {
                ownerHeight
            };
            facebook_yoga_NodeImpl::relativePosition(
                self,
                _axis,
                directionRespectingRoot,
                _axisSize,
            )
        });
        let mut relativePositionCross: f32 = ({
            facebook_yoga_NodeImpl::relativePosition(
                self,
                crossAxis,
                directionRespectingRoot,
                if ({ isRow_128(mainAxis) }) {
                    ownerHeight
                } else {
                    ownerWidth
                },
            )
        });
        let mut mainAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineStartEdge_134(mainAxis, direction) });
        let mut mainAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineEndEdge_135(mainAxis, direction) });
        let mut crossAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineStartEdge_134(crossAxis, direction) });
        let mut crossAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineEndEdge_135(crossAxis, direction) });
        ({
            let _position: f32 = (({
                facebook_yoga_StyleImpl::computeInlineStartMargin(
                    &field_ptr!((*self), style_),
                    mainAxis,
                    direction,
                    ownerWidth,
                )
            }) + relativePositionMain);
            facebook_yoga_NodeImpl::setLayoutPosition(self, _position, mainAxisLeadingEdge)
        });
        ({
            let _position: f32 = (({
                facebook_yoga_StyleImpl::computeInlineEndMargin(
                    &field_ptr!((*self), style_),
                    mainAxis,
                    direction,
                    ownerWidth,
                )
            }) + relativePositionMain);
            facebook_yoga_NodeImpl::setLayoutPosition(self, _position, mainAxisTrailingEdge)
        });
        ({
            let _position: f32 = (({
                facebook_yoga_StyleImpl::computeInlineStartMargin(
                    &field_ptr!((*self), style_),
                    crossAxis,
                    direction,
                    ownerWidth,
                )
            }) + relativePositionCross);
            facebook_yoga_NodeImpl::setLayoutPosition(self, _position, crossAxisLeadingEdge)
        });
        ({
            let _position: f32 = (({
                facebook_yoga_StyleImpl::computeInlineEndMargin(
                    &field_ptr!((*self), style_),
                    crossAxis,
                    direction,
                    ownerWidth,
                )
            }) + relativePositionCross);
            facebook_yoga_NodeImpl::setLayoutPosition(self, _position, crossAxisTrailingEdge)
        });
    }
    fn processFlexBasis(&self) -> facebook_yoga_StyleSizeLength {
        let flexBasis: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
            ({ facebook_yoga_StyleImpl::flexBasis(&field_ptr!((*self), style_)) }),
        ));
        if (!({ facebook_yoga_StyleSizeLengthImpl::isAuto(&flexBasis.as_pointer()) }))
            && (!({ facebook_yoga_StyleSizeLengthImpl::isUndefined(&flexBasis.as_pointer()) }))
        {
            return (*flexBasis.borrow()).clone();
        }
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) && (({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) > 0_f32)
        {
            return if ({
                facebook_yoga_ConfigImpl::useWebDefaults(&(*self).with(|__s| __s.config_.clone()))
            }) {
                ({ facebook_yoga_StyleSizeLength::ofAuto() })
            } else {
                ({ facebook_yoga_StyleSizeLength::points(0_f32) })
            };
        }
        return ({ facebook_yoga_StyleSizeLength::ofAuto() });
    }
    fn resolveFlexBasis(
        &self,
        mut direction: facebook_yoga_Direction,
        mut flexDirection: facebook_yoga_FlexDirection,
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        let mut value: facebook_yoga_FloatOptional = ({
            facebook_yoga_StyleSizeLengthImpl::resolve(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_NodeImpl::processFlexBasis(self) }),
                ))
                .as_pointer(),
                referenceLength,
            )
        });
        if (({ facebook_yoga_StyleImpl::boxSizing(&field_ptr!((*self), style_)) })
            == facebook_yoga_BoxSizing_BorderBox)
        {
            return value;
        }
        let mut dim: facebook_yoga_Dimension = ({ dimension_136(flexDirection) });
        let dimensionPaddingAndBorder: Value<facebook_yoga_FloatOptional> =
            Rc::new(RefCell::new(facebook_yoga_FloatOptional::new_1({
                ({
                    facebook_yoga_StyleImpl::computePaddingAndBorderForDimension(
                        &field_ptr!((*self), style_),
                        direction,
                        dim,
                        ownerWidth,
                    )
                })
            })));
        return ({
            let _lhs: facebook_yoga_FloatOptional = value;
            operator_add_117(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    fn processDimensions(&self) {
        let __tmp_0: Value<Vec<facebook_yoga_Dimension>> = Rc::new(RefCell::new(Vec::from([
            facebook_yoga_Dimension_Width,
            facebook_yoga_Dimension_Height,
        ])));
        let __range2: Ptr<Vec<facebook_yoga_Dimension>> = __tmp_0.as_pointer();
        'loop_: for dim in IterRange::new(__range2.decay(), __range2.decay().to_end()) {
            let mut dim: facebook_yoga_Dimension = (dim.read());
            if ({
                facebook_yoga_StyleSizeLengthImpl::isDefined(
                    &Rc::new(RefCell::new(
                        ({
                            facebook_yoga_StyleImpl::maxDimension(&field_ptr!((*self), style_), dim)
                        }),
                    ))
                    .as_pointer(),
                )
            }) && ({
                let _a: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::maxDimension(&field_ptr!((*self), style_), dim) }),
                ));
                let _b: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::minDimension(&field_ptr!((*self), style_), dim) }),
                ));
                inexactEquals_171(_a.as_pointer(), _b.as_pointer())
            }) {
                let __rhs =
                    ({ facebook_yoga_StyleImpl::maxDimension(&field_ptr!((*self), style_), dim) });
                field_ref!((*self), processedDimensions_)
                    .decay()
                    .offset((({ to_underlying_47(dim) }) as usize) as isize)
                    .write(__rhs);
            } else {
                let __rhs =
                    ({ facebook_yoga_StyleImpl::dimension(&field_ptr!((*self), style_), dim) });
                field_ref!((*self), processedDimensions_)
                    .decay()
                    .offset((({ to_underlying_47(dim) }) as usize) as isize)
                    .write(__rhs);
            }
        }
    }
    fn resolveDirection(
        &self,
        mut ownerDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_Direction {
        if (({ facebook_yoga_StyleImpl::direction(&field_ptr!((*self), style_)) })
            == facebook_yoga_Direction_Inherit)
        {
            return if (ownerDirection != facebook_yoga_Direction_Inherit) {
                ownerDirection
            } else {
                facebook_yoga_Direction_LTR
            };
        } else {
            return ({ facebook_yoga_StyleImpl::direction(&field_ptr!((*self), style_)) });
        }
        panic!("ub: non-void function does not return a value")
    }
    fn clearChildren(&self) {
        field!((*self), children_).with_mut(|__v: &mut Vec<Ptr<facebook_yoga_Node>>| __v.clear());
        field!((*self), children_)
            .with_mut(|__v: &mut Vec<Ptr<facebook_yoga_Node>>| __v.shrink_to_fit());
    }
    fn cloneChildrenIfNeeded(&self) {
        let mut i: usize = 0_usize;
        'loop_: for child in IterRange::new(
            field_ref!((*self), children_).decay(),
            field_ref!((*self), children_).decay().to_end(),
        ) {
            let child: Ptr<Ptr<facebook_yoga_Node>> = (child).clone();
            if (({ facebook_yoga_NodeImpl::getOwner(&(child.read())) }) != (*self)) {
                let __rhs = ({
                    resolveRef_180(
                        ({
                            let _owner: Ptr<YGNode> = (*self).reinterpret_cast::<YGNode>();
                            facebook_yoga_ConfigImpl::cloneNode(
                                &(*self).with(|__s| __s.config_.clone()),
                                (child.read()).reinterpret_cast::<YGNode>(),
                                _owner,
                                i,
                            )
                        }),
                    )
                });
                child.write(__rhs);
                ({
                    let _owner: Ptr<facebook_yoga_Node> = (*self).clone();
                    facebook_yoga_NodeImpl::setOwner(&(child.read()), _owner)
                });
                if (({
                    facebook_yoga_StyleImpl::display(
                        &({ facebook_yoga_NodeImpl::style_24(&(child.read())) }),
                    )
                }) == facebook_yoga_Display_Contents)
                {
                    {
                        ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&(child.read())) });
                    };
                } else if ({ facebook_yoga_NodeImpl::hasContentsChildren(&(child.read())) }) {
                    {
                        ({
                            facebook_yoga_NodeImpl::cloneContentsChildrenIfNeeded(&(child.read()))
                        });
                    };
                }
            }
            i = { (i).wrapping_add(1_usize) };
        }
    }
    fn cloneContentsChildrenIfNeeded(&self) {
        let mut i: usize = 0_usize;
        'loop_: for child in IterRange::new(
            field_ref!((*self), children_).decay(),
            field_ref!((*self), children_).decay().to_end(),
        ) {
            let child: Ptr<Ptr<facebook_yoga_Node>> = (child).clone();
            if (({
                facebook_yoga_StyleImpl::display(
                    &({ facebook_yoga_NodeImpl::style_24(&(child.read())) }),
                )
            }) == facebook_yoga_Display_Contents)
                && (({ facebook_yoga_NodeImpl::getOwner(&(child.read())) }) != (*self))
            {
                let __rhs = ({
                    resolveRef_180(
                        ({
                            let _owner: Ptr<YGNode> = (*self).reinterpret_cast::<YGNode>();
                            facebook_yoga_ConfigImpl::cloneNode(
                                &(*self).with(|__s| __s.config_.clone()),
                                (child.read()).reinterpret_cast::<YGNode>(),
                                _owner,
                                i,
                            )
                        }),
                    )
                });
                child.write(__rhs);
                ({
                    let _owner: Ptr<facebook_yoga_Node> = (*self).clone();
                    facebook_yoga_NodeImpl::setOwner(&(child.read()), _owner)
                });
                ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&(child.read())) });
            }
            i = { (i).wrapping_add(1_usize) };
        }
    }
    fn markDirtyAndPropagate(&self) {
        if !((*self).with(|__s| __s.isDirty_)) {
            ({ facebook_yoga_NodeImpl::setDirty(self, true) });
            ({
                facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(
                    self,
                    <facebook_yoga_FloatOptional>::default(),
                )
            });
            if !(((*self).with(|__s| __s.owner_.clone())).is_null()) {
                ({
                    facebook_yoga_NodeImpl::markDirtyAndPropagate(
                        &(*self).with(|__s| __s.owner_.clone()),
                    )
                });
            }
        }
    }
    fn resolveFlexGrow(&self) -> f32 {
        if ((*self).with(|__s| __s.owner_.clone())).is_null() {
            return (0_f64 as f32);
        }
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flexGrow(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) {
            return ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_StyleImpl::flexGrow(&field_ptr!((*self), style_)) }),
                    ))
                    .as_pointer(),
                )
            });
        }
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) && (({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) > 0_f32)
        {
            return ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                    ))
                    .as_pointer(),
                )
            });
        }
        return DefaultFlexGrow_177.with(|rc| *rc.borrow());
    }
    fn resolveFlexShrink(&self) -> f32 {
        if ((*self).with(|__s| __s.owner_.clone())).is_null() {
            return (0_f64 as f32);
        }
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flexShrink(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) {
            return ({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_StyleImpl::flexShrink(&field_ptr!((*self), style_)) }),
                    ))
                    .as_pointer(),
                )
            });
        }
        if ((!({
            facebook_yoga_ConfigImpl::useWebDefaults(&(*self).with(|__s| __s.config_.clone()))
        })) && ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        })) && (({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                ))
                .as_pointer(),
            )
        }) < 0_f32)
        {
            return -({
                facebook_yoga_FloatOptionalImpl::unwrap(
                    &Rc::new(RefCell::new(
                        ({ facebook_yoga_StyleImpl::flex(&field_ptr!((*self), style_)) }),
                    ))
                    .as_pointer(),
                )
            });
        }
        return if ({
            facebook_yoga_ConfigImpl::useWebDefaults(&(*self).with(|__s| __s.config_.clone()))
        }) {
            WebDefaultFlexShrink_179.with(|rc| *rc.borrow())
        } else {
            DefaultFlexShrink_178.with(|rc| *rc.borrow())
        };
    }
    fn isNodeFlexible(&self) -> bool {
        return ((({ facebook_yoga_StyleImpl::positionType(&field_ptr!((*self), style_)) })
            != facebook_yoga_PositionType_Absolute)
            && ((({ facebook_yoga_NodeImpl::resolveFlexGrow(self) }) != 0_f32)
                || (({ facebook_yoga_NodeImpl::resolveFlexShrink(self) }) != 0_f32)));
    }
    fn reset(&self) {
        ({
            let _node: Ptr<facebook_yoga_Node> = (*self).clone();
            let _condition: bool = (*(*self).upgrade().deref()).children_.is_empty();
            assertFatalWithNode_213(
                _node,
                _condition,
                Ptr::<i8>::from_string_literal(
                    b"Cannot reset a node which still has children attached",
                ),
            )
        });
        ({
            let _node: Ptr<facebook_yoga_Node> = (*self).clone();
            let _condition: bool = ((*self).with(|__s| __s.owner_.clone())).is_null();
            assertFatalWithNode_213(
                _node,
                _condition,
                Ptr::<i8>::from_string_literal(b"Cannot reset a node still attached to a owner"),
            )
        });
        (*self).write(facebook_yoga_Node::new_1({
            ({ facebook_yoga_NodeImpl::getConfig(self) })
        }));
    }
}
pub trait facebook_yoga_SmallValueBuffer_4_Impl {
    fn push_4(&self, value: u32) -> u16;
    fn push_5(&self, mut value: u64) -> u16 {
        unimplemented!()
    }
    fn replace_6(&self, index: u16, value: u32) -> u16;
    fn replace_7(&self, mut index: u16, mut value: u64) -> u16 {
        unimplemented!()
    }
    fn get32(&self, index: u16) -> u32;
    fn get64(&self, mut index: u16) -> u64 {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        other: Ptr<facebook_yoga_SmallValueBuffer_4_>,
    ) -> Ptr<facebook_yoga_SmallValueBuffer_4_>;
    fn move_assign(
        &self,
        other: Ptr<facebook_yoga_SmallValueBuffer_4_>,
    ) -> Ptr<facebook_yoga_SmallValueBuffer_4_>;
}
impl facebook_yoga_SmallValueBuffer_4_Impl for Ptr<facebook_yoga_SmallValueBuffer_4_> {
    fn push_4(&self, mut value: u32) -> u16 {
        let mut index: u16 = field!((*self), count_).with_mut(|__v| __v.postfix_inc());
        assert!(
            ((index as i32) < 4096)
                && (!(Ptr::<i8>::from_string_literal(
                    b"SmallValueBuffer can only hold up to 4096 chunks"
                ))
                .is_null())
        );
        if ((index as usize) < (*(*self).upgrade().deref()).buffer_.len()) {
            field_ref!((*self), buffer_)
                .decay()
                .offset((index as usize) as isize)
                .write(value);
            return index;
        }
        if (*self).with(|__s| __s.overflow_.clone()).is_none() {
            field_ptr!((*self), overflow_).write(
                Some(Rc::new(RefCell::new(libcc2rs::cpp_make!(
                    facebook_yoga_SmallValueBuffer_4__Overflow,
                ))))
                .take(),
            );
        }
        field!(
            (*self).with(|__s| __s.overflow_.clone()).as_pointer(),
            buffer_
        )
        .with_mut(|__v: &mut Vec<u32>| __v.push(value.clone()));
        field!(
            (*self).with(|__s| __s.overflow_.clone()).as_pointer(),
            wideElements_
        )
        .with_mut(|__v: &mut Vec<bool>| __v.push(false));
        return index;
    }
    fn replace_6(&self, mut index: u16, mut value: u32) -> u16 {
        if ((index as usize) < (*(*self).upgrade().deref()).buffer_.len()) {
            field_ref!((*self), buffer_)
                .decay()
                .offset((index as usize) as isize)
                .write(value);
        } else {
            field_ref!(
                (*self).with(|__s| __s.overflow_.clone()).as_pointer(),
                buffer_
            )
            .decay()
            .offset(
                (index as usize).wrapping_sub((*(*self).upgrade().deref()).buffer_.len()) as isize,
            )
            .write(value);
        }
        return index;
    }
    fn get32(&self, mut index: u16) -> u32 {
        if ((index as usize) < (*(*self).upgrade().deref()).buffer_.len()) {
            return (field_ref!((*self), buffer_)
                .decay()
                .offset((index as usize) as isize)
                .read());
        } else {
            return (field_ref!(
                (*self).with(|__s| __s.overflow_.clone()).as_pointer(),
                buffer_
            )
            .decay()
            .offset(
                (index as usize).wrapping_sub((*(*self).upgrade().deref()).buffer_.len()) as isize,
            )
            .read());
        }
        panic!("ub: non-void function does not return a value")
    }
    fn copy_assign(
        &self,
        other: Ptr<facebook_yoga_SmallValueBuffer_4_>,
    ) -> Ptr<facebook_yoga_SmallValueBuffer_4_> {
        field!((*self), count_).write({ other.with(|__s| __s.count_) });
        field!((*self), buffer_).write({ other.with(|__s| __s.buffer_.clone()) });
        field!((*self), wideElements_).write({ other.with(|__s| __s.wideElements_.clone()) });
        field_ptr!((*self), overflow_).write(
            if other.with(|__s| __s.overflow_.clone()).is_some() {
                Some(Rc::new(RefCell::new(libcc2rs::cpp_make!(
                    facebook_yoga_SmallValueBuffer_4__Overflow,
                    ({
                        (*other
                            .with(|__s| __s.overflow_.clone())
                            .as_pointer()
                            .upgrade()
                            .deref())
                        .clone()
                    })
                ))))
            } else {
                None
            }
            .take(),
        );
        return (*self).clone();
    }
    fn move_assign(
        &self,
        other: Ptr<facebook_yoga_SmallValueBuffer_4_>,
    ) -> Ptr<facebook_yoga_SmallValueBuffer_4_> {
        field!((*self), count_).write({ { (*other.upgrade().deref()).count_ } });
        field!((*self), buffer_).write({ { (*other.upgrade().deref()).buffer_.clone() } });
        field!((*self), wideElements_)
            .write({ { (*other.upgrade().deref()).wideElements_.clone() } });
        field_ptr!((*self), overflow_).write(field!(other, overflow_).with_mut(
            |__v: &mut Option<Value<facebook_yoga_SmallValueBuffer_4__Overflow>>| __v.take(),
        ));
        return (*self).clone();
    }
}
pub trait facebook_yoga_SmallValueBuffer_4__OverflowImpl {
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_SmallValueBuffer_4__Overflow>,
    ) -> Ptr<facebook_yoga_SmallValueBuffer_4__Overflow>;
}
impl facebook_yoga_SmallValueBuffer_4__OverflowImpl
    for Ptr<facebook_yoga_SmallValueBuffer_4__Overflow>
{
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_SmallValueBuffer_4__Overflow>,
    ) -> Ptr<facebook_yoga_SmallValueBuffer_4__Overflow> {
        field_ref!((*self), buffer_)
            .as_pointer()
            .write(field!(_a0, buffer_).with_mut(std::mem::take));
        field_ref!((*self), wideElements_)
            .as_pointer()
            .write(field!(_a0, wideElements_).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub trait facebook_yoga_StyleImpl {
    fn direction(&self) -> facebook_yoga_Direction;
    fn setDirection(&self, value: facebook_yoga_Direction);
    fn flexDirection(&self) -> facebook_yoga_FlexDirection;
    fn setFlexDirection(&self, value: facebook_yoga_FlexDirection);
    fn justifyContent(&self) -> facebook_yoga_Justify;
    fn setJustifyContent(&self, value: facebook_yoga_Justify);
    fn justifyItems(&self) -> facebook_yoga_Justify;
    fn setJustifyItems(&self, value: facebook_yoga_Justify);
    fn justifySelf(&self) -> facebook_yoga_Justify;
    fn setJustifySelf(&self, value: facebook_yoga_Justify);
    fn alignContent(&self) -> facebook_yoga_Align;
    fn setAlignContent(&self, value: facebook_yoga_Align);
    fn alignItems(&self) -> facebook_yoga_Align;
    fn setAlignItems(&self, value: facebook_yoga_Align);
    fn alignSelf(&self) -> facebook_yoga_Align;
    fn setAlignSelf(&self, value: facebook_yoga_Align);
    fn positionType(&self) -> facebook_yoga_PositionType;
    fn setPositionType(&self, value: facebook_yoga_PositionType);
    fn flexWrap(&self) -> facebook_yoga_Wrap;
    fn setFlexWrap(&self, value: facebook_yoga_Wrap);
    fn overflow(&self) -> facebook_yoga_Overflow;
    fn setOverflow(&self, value: facebook_yoga_Overflow);
    fn display(&self) -> facebook_yoga_Display;
    fn setDisplay(&self, value: facebook_yoga_Display);
    fn flex(&self) -> facebook_yoga_FloatOptional;
    fn setFlex(&self, value: facebook_yoga_FloatOptional);
    fn flexGrow(&self) -> facebook_yoga_FloatOptional;
    fn setFlexGrow(&self, value: facebook_yoga_FloatOptional);
    fn flexShrink(&self) -> facebook_yoga_FloatOptional;
    fn setFlexShrink(&self, value: facebook_yoga_FloatOptional);
    fn flexBasis(&self) -> facebook_yoga_StyleSizeLength;
    fn setFlexBasis(&self, value: facebook_yoga_StyleSizeLength);
    fn margin(&self, edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength;
    fn setMargin(&self, edge: facebook_yoga_Edge, value: facebook_yoga_StyleLength);
    fn position(&self, edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength;
    fn setPosition(&self, edge: facebook_yoga_Edge, value: facebook_yoga_StyleLength);
    fn padding(&self, edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength;
    fn setPadding(&self, edge: facebook_yoga_Edge, value: facebook_yoga_StyleLength);
    fn border(&self, edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength;
    fn setBorder(&self, edge: facebook_yoga_Edge, value: facebook_yoga_StyleLength);
    fn gap(&self, gutter: facebook_yoga_Gutter) -> facebook_yoga_StyleLength;
    fn setGap(&self, gutter: facebook_yoga_Gutter, value: facebook_yoga_StyleLength);
    fn dimension(&self, axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength;
    fn setDimension(&self, axis: facebook_yoga_Dimension, value: facebook_yoga_StyleSizeLength);
    fn minDimension(&self, axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength;
    fn setMinDimension(&self, axis: facebook_yoga_Dimension, value: facebook_yoga_StyleSizeLength);
    fn gridTemplateColumns(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>>;
    fn setGridTemplateColumns(&self, value: Vec<facebook_yoga_GridTrackSize>);
    fn resizeGridTemplateColumns(&self, count: usize);
    fn setGridTemplateColumnAt(&self, index: usize, value: facebook_yoga_GridTrackSize);
    fn gridTemplateRows(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>>;
    fn setGridTemplateRows(&self, value: Vec<facebook_yoga_GridTrackSize>);
    fn resizeGridTemplateRows(&self, count: usize);
    fn setGridTemplateRowAt(&self, index: usize, value: facebook_yoga_GridTrackSize);
    fn gridAutoColumns(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>>;
    fn setGridAutoColumns(&self, value: Vec<facebook_yoga_GridTrackSize>);
    fn resizeGridAutoColumns(&self, count: usize);
    fn setGridAutoColumnAt(&self, index: usize, value: facebook_yoga_GridTrackSize);
    fn gridAutoRows(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>>;
    fn setGridAutoRows(&self, value: Vec<facebook_yoga_GridTrackSize>);
    fn resizeGridAutoRows(&self, count: usize);
    fn setGridAutoRowAt(&self, index: usize, value: facebook_yoga_GridTrackSize);
    fn gridColumnStart(&self) -> Ptr<facebook_yoga_GridLine>;
    fn setGridColumnStart(&self, value: facebook_yoga_GridLine);
    fn gridColumnEnd(&self) -> Ptr<facebook_yoga_GridLine>;
    fn setGridColumnEnd(&self, value: facebook_yoga_GridLine);
    fn gridRowStart(&self) -> Ptr<facebook_yoga_GridLine>;
    fn setGridRowStart(&self, value: facebook_yoga_GridLine);
    fn gridRowEnd(&self) -> Ptr<facebook_yoga_GridLine>;
    fn setGridRowEnd(&self, value: facebook_yoga_GridLine);
    fn resolvedMinDimension(
        &self,
        direction: facebook_yoga_Direction,
        axis: facebook_yoga_Dimension,
        referenceLength: f32,
        ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional;
    fn maxDimension(&self, axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength;
    fn setMaxDimension(&self, axis: facebook_yoga_Dimension, value: facebook_yoga_StyleSizeLength);
    fn resolvedMaxDimension(
        &self,
        direction: facebook_yoga_Direction,
        axis: facebook_yoga_Dimension,
        referenceLength: f32,
        ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional;
    fn resolvesAgainstOwnerWidth(&self) -> bool;
    fn resolvesAgainstOwnerHeight(&self) -> bool;
    fn aspectRatio(&self) -> facebook_yoga_FloatOptional;
    fn setAspectRatio(&self, value: facebook_yoga_FloatOptional);
    fn boxSizing(&self) -> facebook_yoga_BoxSizing;
    fn setBoxSizing(&self, value: facebook_yoga_BoxSizing);
    fn horizontalInsetsDefined(&self) -> bool;
    fn verticalInsetsDefined(&self) -> bool;
    fn isFlexStartPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isFlexStartPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isInlineStartPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isInlineStartPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isFlexEndPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isFlexEndPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isInlineEndPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn isInlineEndPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn computeFlexStartPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: f32,
    ) -> f32;
    fn computeInlineStartPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: f32,
    ) -> f32;
    fn computeFlexEndPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: f32,
    ) -> f32;
    fn computeInlineEndPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: f32,
    ) -> f32;
    fn computeFlexStartMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeInlineStartMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeFlexEndMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeInlineEndMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeFlexStartBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> f32;
    fn computeInlineStartBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> f32;
    fn computeFlexEndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> f32;
    fn computeInlineEndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> f32;
    fn computeFlexStartPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeInlineStartPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeFlexEndPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeInlineEndPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeInlineStartPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeFlexStartPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeInlineEndPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computeFlexEndPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: f32,
    ) -> f32;
    fn computePaddingAndBorderForDimension(
        &self,
        direction: facebook_yoga_Direction,
        dimension: facebook_yoga_Dimension,
        widthSize: f32,
    ) -> f32;
    fn computeBorderForAxis(&self, axis: facebook_yoga_FlexDirection) -> f32;
    fn computeMarginForAxis(&self, axis: facebook_yoga_FlexDirection, widthSize: f32) -> f32;
    fn computeGapForAxis(&self, axis: facebook_yoga_FlexDirection, ownerSize: f32) -> f32;
    fn computeGapForDimension(&self, dimension: facebook_yoga_Dimension, ownerSize: f32) -> f32;
    fn flexStartMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn flexEndMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn inlineStartMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn inlineEndMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> bool;
    fn operator_eq(&self, other: Ptr<facebook_yoga_Style>) -> bool;
    fn computeColumnGap(&self) -> facebook_yoga_StyleValueHandle;
    fn computeRowGap(&self) -> facebook_yoga_StyleValueHandle;
    fn computeLeftEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeTopEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeRightEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeBottomEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
    ) -> facebook_yoga_StyleValueHandle;
    fn computePosition(
        &self,
        edge: facebook_yoga_PhysicalEdge,
        direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeMargin(
        &self,
        edge: facebook_yoga_PhysicalEdge,
        direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computePadding(
        &self,
        edge: facebook_yoga_PhysicalEdge,
        direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeBorder(
        &self,
        edge: facebook_yoga_PhysicalEdge,
        direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn resolve(
        &self,
        handle: facebook_yoga_StyleValueHandle,
        referenceLength: f32,
    ) -> facebook_yoga_FloatOptional;
    fn copy_assign(&self, _a0: Ptr<facebook_yoga_Style>) -> Ptr<facebook_yoga_Style>;
    fn move_assign(&self, _a0: Ptr<facebook_yoga_Style>) -> Ptr<facebook_yoga_Style>;
}
impl facebook_yoga_StyleImpl for Ptr<facebook_yoga_Style> {
    fn direction(&self) -> facebook_yoga_Direction {
        return (*self).with(|__s| __s.direction_);
    }
    fn setDirection(&self, mut value: facebook_yoga_Direction) {
        field!((*self), direction_).write(value);
    }
    fn flexDirection(&self) -> facebook_yoga_FlexDirection {
        return (*self).with(|__s| __s.flexDirection_);
    }
    fn setFlexDirection(&self, mut value: facebook_yoga_FlexDirection) {
        field!((*self), flexDirection_).write(value);
    }
    fn justifyContent(&self) -> facebook_yoga_Justify {
        return (*self).with(|__s| __s.justifyContent_);
    }
    fn setJustifyContent(&self, mut value: facebook_yoga_Justify) {
        field!((*self), justifyContent_).write(value);
    }
    fn justifyItems(&self) -> facebook_yoga_Justify {
        return (*self).with(|__s| __s.justifyItems_);
    }
    fn setJustifyItems(&self, mut value: facebook_yoga_Justify) {
        field!((*self), justifyItems_).write(value);
    }
    fn justifySelf(&self) -> facebook_yoga_Justify {
        return (*self).with(|__s| __s.justifySelf_);
    }
    fn setJustifySelf(&self, mut value: facebook_yoga_Justify) {
        field!((*self), justifySelf_).write(value);
    }
    fn alignContent(&self) -> facebook_yoga_Align {
        return (*self).with(|__s| __s.alignContent_);
    }
    fn setAlignContent(&self, mut value: facebook_yoga_Align) {
        field!((*self), alignContent_).write(value);
    }
    fn alignItems(&self) -> facebook_yoga_Align {
        return (*self).with(|__s| __s.alignItems_);
    }
    fn setAlignItems(&self, mut value: facebook_yoga_Align) {
        field!((*self), alignItems_).write(value);
    }
    fn alignSelf(&self) -> facebook_yoga_Align {
        return (*self).with(|__s| __s.alignSelf_);
    }
    fn setAlignSelf(&self, mut value: facebook_yoga_Align) {
        field!((*self), alignSelf_).write(value);
    }
    fn positionType(&self) -> facebook_yoga_PositionType {
        return (*self).with(|__s| __s.positionType_);
    }
    fn setPositionType(&self, mut value: facebook_yoga_PositionType) {
        field!((*self), positionType_).write(value);
    }
    fn flexWrap(&self) -> facebook_yoga_Wrap {
        return (*self).with(|__s| __s.flexWrap_);
    }
    fn setFlexWrap(&self, mut value: facebook_yoga_Wrap) {
        field!((*self), flexWrap_).write(value);
    }
    fn overflow(&self) -> facebook_yoga_Overflow {
        return (*self).with(|__s| __s.overflow_);
    }
    fn setOverflow(&self, mut value: facebook_yoga_Overflow) {
        field!((*self), overflow_).write(value);
    }
    fn display(&self) -> facebook_yoga_Display {
        return (*self).with(|__s| __s.display_);
    }
    fn setDisplay(&self, mut value: facebook_yoga_Display) {
        field!((*self), display_).write(value);
    }
    fn flex(&self) -> facebook_yoga_FloatOptional {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*self).with(|__s| __s.flex_.clone());
            facebook_yoga_StyleValuePoolImpl::getNumber(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setFlex(&self, mut value: facebook_yoga_FloatOptional) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!((*self), flex_);
            facebook_yoga_StyleValuePoolImpl::store_3(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn flexGrow(&self) -> facebook_yoga_FloatOptional {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*self).with(|__s| __s.flexGrow_.clone());
            facebook_yoga_StyleValuePoolImpl::getNumber(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setFlexGrow(&self, mut value: facebook_yoga_FloatOptional) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!((*self), flexGrow_);
            facebook_yoga_StyleValuePoolImpl::store_3(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn flexShrink(&self) -> facebook_yoga_FloatOptional {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*self).with(|__s| __s.flexShrink_.clone());
            facebook_yoga_StyleValuePoolImpl::getNumber(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setFlexShrink(&self, mut value: facebook_yoga_FloatOptional) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!((*self), flexShrink_);
            facebook_yoga_StyleValuePoolImpl::store_3(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn flexBasis(&self) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*self).with(|__s| __s.flexBasis_.clone());
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setFlexBasis(&self, mut value: facebook_yoga_StyleSizeLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!((*self), flexBasis_);
            facebook_yoga_StyleValuePoolImpl::store_2(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn margin(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), margin_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setMargin(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), margin_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn position(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), position_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setPosition(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), position_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn padding(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), padding_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setPadding(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), padding_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn border(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), border_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setBorder(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), border_)
                .decay()
                .offset((({ to_underlying_46(edge) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn gap(&self, mut gutter: facebook_yoga_Gutter) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), gap_)
                .decay()
                .offset((({ to_underlying_49(gutter) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setGap(&self, mut gutter: facebook_yoga_Gutter, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), gap_)
                .decay()
                .offset((({ to_underlying_49(gutter) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn dimension(&self, mut axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), dimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), dimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_2(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn minDimension(&self, mut axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), minDimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setMinDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), minDimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_2(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn gridTemplateColumns(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>> {
        return field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            templateColumns
        )
        .as_pointer();
    }
    fn setGridTemplateColumns(&self, value: Vec<facebook_yoga_GridTrackSize>) {
        let value: Value<Vec<facebook_yoga_GridTrackSize>> = Rc::new(RefCell::new(value));
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            templateColumns
        )
        .as_pointer()
        .write(({ std::mem::take(&mut (*value.borrow_mut())) }));
    }
    fn resizeGridTemplateColumns(&self, mut count: usize) {
        {
            let __n = count as usize;
            field!(
                ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
                templateColumns
            )
            .with_mut(|__v: &mut Vec<facebook_yoga_GridTrackSize>| {
                __v.resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
            })
        };
    }
    fn setGridTemplateColumnAt(&self, mut index: usize, mut value: facebook_yoga_GridTrackSize) {
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            templateColumns
        )
        .decay()
        .offset(index as isize)
        .write((value).clone());
    }
    fn gridTemplateRows(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>> {
        return field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            templateRows
        )
        .as_pointer();
    }
    fn setGridTemplateRows(&self, value: Vec<facebook_yoga_GridTrackSize>) {
        let value: Value<Vec<facebook_yoga_GridTrackSize>> = Rc::new(RefCell::new(value));
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            templateRows
        )
        .as_pointer()
        .write(({ std::mem::take(&mut (*value.borrow_mut())) }));
    }
    fn resizeGridTemplateRows(&self, mut count: usize) {
        {
            let __n = count as usize;
            field!(
                ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
                templateRows
            )
            .with_mut(|__v: &mut Vec<facebook_yoga_GridTrackSize>| {
                __v.resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
            })
        };
    }
    fn setGridTemplateRowAt(&self, mut index: usize, mut value: facebook_yoga_GridTrackSize) {
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            templateRows
        )
        .decay()
        .offset(index as isize)
        .write((value).clone());
    }
    fn gridAutoColumns(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>> {
        return field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            autoColumns
        )
        .as_pointer();
    }
    fn setGridAutoColumns(&self, value: Vec<facebook_yoga_GridTrackSize>) {
        let value: Value<Vec<facebook_yoga_GridTrackSize>> = Rc::new(RefCell::new(value));
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            autoColumns
        )
        .as_pointer()
        .write(({ std::mem::take(&mut (*value.borrow_mut())) }));
    }
    fn resizeGridAutoColumns(&self, mut count: usize) {
        {
            let __n = count as usize;
            field!(
                ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
                autoColumns
            )
            .with_mut(|__v: &mut Vec<facebook_yoga_GridTrackSize>| {
                __v.resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
            })
        };
    }
    fn setGridAutoColumnAt(&self, mut index: usize, mut value: facebook_yoga_GridTrackSize) {
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            autoColumns
        )
        .decay()
        .offset(index as isize)
        .write((value).clone());
    }
    fn gridAutoRows(&self) -> Ptr<Vec<facebook_yoga_GridTrackSize>> {
        return field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            autoRows
        )
        .as_pointer();
    }
    fn setGridAutoRows(&self, value: Vec<facebook_yoga_GridTrackSize>) {
        let value: Value<Vec<facebook_yoga_GridTrackSize>> = Rc::new(RefCell::new(value));
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            autoRows
        )
        .as_pointer()
        .write(({ std::mem::take(&mut (*value.borrow_mut())) }));
    }
    fn resizeGridAutoRows(&self, mut count: usize) {
        {
            let __n = count as usize;
            field!(
                ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
                autoRows
            )
            .with_mut(|__v: &mut Vec<facebook_yoga_GridTrackSize>| {
                __v.resize_with(__n, || <facebook_yoga_GridTrackSize>::default())
            })
        };
    }
    fn setGridAutoRowAt(&self, mut index: usize, mut value: facebook_yoga_GridTrackSize) {
        field_ref!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            autoRows
        )
        .decay()
        .offset(index as isize)
        .write((value).clone());
    }
    fn gridColumnStart(&self) -> Ptr<facebook_yoga_GridLine> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            columnStart
        );
    }
    fn setGridColumnStart(&self, mut value: facebook_yoga_GridLine) {
        field!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            columnStart
        )
        .write((value).clone());
    }
    fn gridColumnEnd(&self) -> Ptr<facebook_yoga_GridLine> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            columnEnd
        );
    }
    fn setGridColumnEnd(&self, mut value: facebook_yoga_GridLine) {
        field!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            columnEnd
        )
        .write((value).clone());
    }
    fn gridRowStart(&self) -> Ptr<facebook_yoga_GridLine> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            rowStart
        );
    }
    fn setGridRowStart(&self, mut value: facebook_yoga_GridLine) {
        field!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            rowStart
        )
        .write((value).clone());
    }
    fn gridRowEnd(&self) -> Ptr<facebook_yoga_GridLine> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            rowEnd
        );
    }
    fn setGridRowEnd(&self, mut value: facebook_yoga_GridLine) {
        field!(
            ({ facebook_yoga_GridStyleStorageImpl::ensure(&field_ptr!((*self), grid_),) }),
            rowEnd
        )
        .write((value).clone());
    }
    fn resolvedMinDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut axis: facebook_yoga_Dimension,
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(
            (*field_ref!((*self), minDimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone(),
        ));
        if ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&handle.as_pointer()) }) {
            return <facebook_yoga_FloatOptional>::default();
        }
        let value: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
            ({
                facebook_yoga_StyleImpl::resolve(self, (*handle.borrow()).clone(), referenceLength)
            }),
        ));
        if (({ facebook_yoga_StyleImpl::boxSizing(self) }) == facebook_yoga_BoxSizing_BorderBox)
            || (!({ facebook_yoga_FloatOptionalImpl::isDefined(&value.as_pointer()) }))
        {
            return (*value.borrow()).clone();
        }
        let dimensionPaddingAndBorder: Value<facebook_yoga_FloatOptional> =
            Rc::new(RefCell::new(facebook_yoga_FloatOptional::new_1({
                ({
                    facebook_yoga_StyleImpl::computePaddingAndBorderForDimension(
                        self, direction, axis, ownerWidth,
                    )
                })
            })));
        return ({
            let _lhs: facebook_yoga_FloatOptional = (*value.borrow()).clone();
            operator_add_117(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    fn maxDimension(&self, mut axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle = (*field_ref!((*self), maxDimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setMaxDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ref!((*self), maxDimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize);
            facebook_yoga_StyleValuePoolImpl::store_2(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn resolvedMaxDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut axis: facebook_yoga_Dimension,
        mut referenceLength: f32,
        mut ownerWidth: f32,
    ) -> facebook_yoga_FloatOptional {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(
            (*field_ref!((*self), maxDimensions_)
                .decay()
                .offset((({ to_underlying_47(axis) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone(),
        ));
        if ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&handle.as_pointer()) }) {
            return <facebook_yoga_FloatOptional>::default();
        }
        let value: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
            ({
                facebook_yoga_StyleImpl::resolve(self, (*handle.borrow()).clone(), referenceLength)
            }),
        ));
        if (({ facebook_yoga_StyleImpl::boxSizing(self) }) == facebook_yoga_BoxSizing_BorderBox)
            || (!({ facebook_yoga_FloatOptionalImpl::isDefined(&value.as_pointer()) }))
        {
            return (*value.borrow()).clone();
        }
        let dimensionPaddingAndBorder: Value<facebook_yoga_FloatOptional> =
            Rc::new(RefCell::new(facebook_yoga_FloatOptional::new_1({
                ({
                    facebook_yoga_StyleImpl::computePaddingAndBorderForDimension(
                        self, direction, axis, ownerWidth,
                    )
                })
            })));
        return ({
            let _lhs: facebook_yoga_FloatOptional = (*value.borrow()).clone();
            operator_add_117(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ (0_f64 as f32) })
                }),
            )
        });
    }
    fn resolvesAgainstOwnerWidth(&self) -> bool {
        return (((({
            facebook_yoga_Style::anyIsPercent_121_9(field_ref!((*self), margin_).as_pointer())
        }) || ({
            facebook_yoga_Style::anyIsPercent_121_9(field_ref!((*self), padding_).as_pointer())
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isPercent(
                &field_ref!((*self), dimensions_).decay().offset(
                    (({ to_underlying_47(facebook_yoga_Dimension_Width) }) as usize) as isize,
                ),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isPercent(
                &field_ref!((*self), minDimensions_).decay().offset(
                    (({ to_underlying_47(facebook_yoga_Dimension_Width) }) as usize) as isize,
                ),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isPercent(
                &field_ref!((*self), maxDimensions_).decay().offset(
                    (({ to_underlying_47(facebook_yoga_Dimension_Width) }) as usize) as isize,
                ),
            )
        });
    }
    fn resolvesAgainstOwnerHeight(&self) -> bool {
        return (({
            facebook_yoga_StyleValueHandleImpl::isPercent(
                &field_ref!((*self), dimensions_).decay().offset(
                    (({ to_underlying_47(facebook_yoga_Dimension_Height) }) as usize) as isize,
                ),
            )
        }) || ({
            facebook_yoga_StyleValueHandleImpl::isPercent(
                &field_ref!((*self), minDimensions_).decay().offset(
                    (({ to_underlying_47(facebook_yoga_Dimension_Height) }) as usize) as isize,
                ),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isPercent(
                &field_ref!((*self), maxDimensions_).decay().offset(
                    (({ to_underlying_47(facebook_yoga_Dimension_Height) }) as usize) as isize,
                ),
            )
        });
    }
    fn aspectRatio(&self) -> facebook_yoga_FloatOptional {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*self).with(|__s| __s.aspectRatio_.clone());
            facebook_yoga_StyleValuePoolImpl::getNumber(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setAspectRatio(&self, value: facebook_yoga_FloatOptional) {
        let value: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(value));
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!((*self), aspectRatio_);
            facebook_yoga_StyleValuePoolImpl::store_3(
                &field_ptr!((*self), pool_),
                _handle,
                if ({
                    let _lhs: facebook_yoga_FloatOptional = (*value.borrow()).clone();
                    operator_eq_115(_lhs, 0_f32)
                }) || (({ facebook_yoga_FloatOptionalImpl::unwrap(&value.as_pointer()) })
                    .is_infinite())
                {
                    <facebook_yoga_FloatOptional>::default()
                } else {
                    (*value.borrow()).clone()
                },
            )
        });
    }
    fn boxSizing(&self) -> facebook_yoga_BoxSizing {
        return (*self).with(|__s| __s.boxSizing_);
    }
    fn setBoxSizing(&self, mut value: facebook_yoga_BoxSizing) {
        field!((*self), boxSizing_).write(value);
    }
    fn horizontalInsetsDefined(&self) -> bool {
        return ((((({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Left) }) as usize) as isize),
            )
        }) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Right) }) as usize) as isize),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_All) }) as usize) as isize),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_).decay().offset(
                    (({ to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as isize,
                ),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as isize),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_End) }) as usize) as isize),
            )
        });
    }
    fn verticalInsetsDefined(&self) -> bool {
        return ((({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Top) }) as usize) as isize),
            )
        }) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Bottom) }) as usize) as isize),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_)
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_All) }) as usize) as isize),
            )
        })) || ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), position_).decay().offset(
                    (({ to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as isize,
                ),
            )
        });
    }
    fn isFlexStartPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexStartEdge_132(axis) }),
                            direction,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isFlexStartPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexStartEdge_132(axis) }),
                            direction,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isInlineStartPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineStartEdge_134(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computePosition(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isInlineStartPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineStartEdge_134(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computePosition(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isFlexEndPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexEndEdge_133(axis) }),
                            direction,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isFlexEndPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexEndEdge_133(axis) }),
                            direction,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isInlineEndPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineEndEdge_135(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computePosition(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn isInlineEndPositionAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineEndEdge_135(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computePosition(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn computeFlexStartPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computePosition(
                                self,
                                ({ flexStartEdge_132(axis) }),
                                direction,
                            )
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, axisSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeInlineStartPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineStartEdge_134(axis, direction) });
                            let _direction: facebook_yoga_Direction = direction;
                            facebook_yoga_StyleImpl::computePosition(self, _edge, _direction)
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, axisSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeFlexEndPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computePosition(
                                self,
                                ({ flexEndEdge_133(axis) }),
                                direction,
                            )
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, axisSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeInlineEndPosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineEndEdge_135(axis, direction) });
                            let _direction: facebook_yoga_Direction = direction;
                            facebook_yoga_StyleImpl::computePosition(self, _edge, _direction)
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, axisSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeFlexStartMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computeMargin(
                                self,
                                ({ flexStartEdge_132(axis) }),
                                direction,
                            )
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeInlineStartMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineStartEdge_134(axis, direction) });
                            let _direction: facebook_yoga_Direction = direction;
                            facebook_yoga_StyleImpl::computeMargin(self, _edge, _direction)
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeFlexEndMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computeMargin(
                                self,
                                ({ flexEndEdge_133(axis) }),
                                direction,
                            )
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeInlineEndMargin(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineEndEdge_135(axis, direction) });
                            let _direction: facebook_yoga_Direction = direction;
                            facebook_yoga_StyleImpl::computeMargin(self, _edge, _direction)
                        });
                        facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                    }),
                ))
                .as_pointer(),
                0_f32,
            )
        });
    }
    fn computeFlexStartBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computeBorder(
                                        self,
                                        ({ flexStartEdge_132(axis) }),
                                        direction,
                                    )
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, 0_f32)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeInlineStartBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineStartEdge_134(axis, direction) });
                                    let _direction: facebook_yoga_Direction = direction;
                                    facebook_yoga_StyleImpl::computeBorder(self, _edge, _direction)
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, 0_f32)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeFlexEndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computeBorder(
                                        self,
                                        ({ flexEndEdge_133(axis) }),
                                        direction,
                                    )
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, 0_f32)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeInlineEndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineEndEdge_135(axis, direction) });
                                    let _direction: facebook_yoga_Direction = direction;
                                    facebook_yoga_StyleImpl::computeBorder(self, _edge, _direction)
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, 0_f32)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeFlexStartPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computePadding(
                                        self,
                                        ({ flexStartEdge_132(axis) }),
                                        direction,
                                    )
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeInlineStartPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineStartEdge_134(axis, direction) });
                                    let _direction: facebook_yoga_Direction = direction;
                                    facebook_yoga_StyleImpl::computePadding(self, _edge, _direction)
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeFlexEndPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computePadding(
                                        self,
                                        ({ flexEndEdge_133(axis) }),
                                        direction,
                                    )
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeInlineEndPadding(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineEndEdge_135(axis, direction) });
                                    let _direction: facebook_yoga_Direction = direction;
                                    facebook_yoga_StyleImpl::computePadding(self, _edge, _direction)
                                });
                                facebook_yoga_StyleImpl::resolve(self, _handle, widthSize)
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeInlineStartPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return (({
            facebook_yoga_StyleImpl::computeInlineStartPadding(self, axis, direction, widthSize)
        }) + ({
            facebook_yoga_StyleImpl::computeInlineStartBorder(self, axis, direction)
        }));
    }
    fn computeFlexStartPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return (({
            facebook_yoga_StyleImpl::computeFlexStartPadding(self, axis, direction, widthSize)
        }) + ({ facebook_yoga_StyleImpl::computeFlexStartBorder(self, axis, direction) }));
    }
    fn computeInlineEndPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return (({
            facebook_yoga_StyleImpl::computeInlineEndPadding(self, axis, direction, widthSize)
        }) + ({ facebook_yoga_StyleImpl::computeInlineEndBorder(self, axis, direction) }));
    }
    fn computeFlexEndPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: f32,
    ) -> f32 {
        return (({
            facebook_yoga_StyleImpl::computeFlexEndPadding(self, axis, direction, widthSize)
        }) + ({ facebook_yoga_StyleImpl::computeFlexEndBorder(self, axis, direction) }));
    }
    fn computePaddingAndBorderForDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut widthSize: f32,
    ) -> f32 {
        let mut flexDirectionForDimension: facebook_yoga_FlexDirection =
            if (dimension == facebook_yoga_Dimension_Width) {
                facebook_yoga_FlexDirection_Row
            } else {
                facebook_yoga_FlexDirection_Column
            };
        return (({
            facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
                self,
                flexDirectionForDimension,
                direction,
                widthSize,
            )
        }) + ({
            facebook_yoga_StyleImpl::computeFlexEndPaddingAndBorder(
                self,
                flexDirectionForDimension,
                direction,
                widthSize,
            )
        }));
    }
    fn computeBorderForAxis(&self, mut axis: facebook_yoga_FlexDirection) -> f32 {
        return (({
            facebook_yoga_StyleImpl::computeInlineStartBorder(
                self,
                axis,
                facebook_yoga_Direction_LTR,
            )
        }) + ({
            facebook_yoga_StyleImpl::computeInlineEndBorder(self, axis, facebook_yoga_Direction_LTR)
        }));
    }
    fn computeMarginForAxis(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut widthSize: f32,
    ) -> f32 {
        return (({
            facebook_yoga_StyleImpl::computeInlineStartMargin(
                self,
                axis,
                facebook_yoga_Direction_LTR,
                widthSize,
            )
        }) + ({
            facebook_yoga_StyleImpl::computeInlineEndMargin(
                self,
                axis,
                facebook_yoga_Direction_LTR,
                widthSize,
            )
        }));
    }
    fn computeGapForAxis(&self, mut axis: facebook_yoga_FlexDirection, mut ownerSize: f32) -> f32 {
        let mut gap: facebook_yoga_StyleValueHandle = if ({ isRow_128(axis) }) {
            ({ facebook_yoga_StyleImpl::computeColumnGap(self) })
        } else {
            ({ facebook_yoga_StyleImpl::computeRowGap(self) })
        };
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({ facebook_yoga_StyleImpl::resolve(self, gap, ownerSize) }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn computeGapForDimension(
        &self,
        mut dimension: facebook_yoga_Dimension,
        mut ownerSize: f32,
    ) -> f32 {
        let mut gap: facebook_yoga_StyleValueHandle =
            if (dimension == facebook_yoga_Dimension_Width) {
                ({ facebook_yoga_StyleImpl::computeColumnGap(self) })
            } else {
                ({ facebook_yoga_StyleImpl::computeRowGap(self) })
            };
        return ({
            maxOrDefined_110(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({ facebook_yoga_StyleImpl::resolve(self, gap, ownerSize) }),
                        ))
                        .as_pointer(),
                    )
                }),
                0_f32,
            )
        });
    }
    fn flexStartMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computeMargin(
                            self,
                            ({ flexStartEdge_132(axis) }),
                            direction,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn flexEndMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computeMargin(
                            self,
                            ({ flexEndEdge_133(axis) }),
                            direction,
                        )
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn inlineStartMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineStartEdge_134(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computeMargin(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn inlineEndMarginIsAuto(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> bool {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineEndEdge_135(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computeMargin(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn operator_eq(&self, other: Ptr<facebook_yoga_Style>) -> bool {
        return (((((((((((((((((((((((((({
            (*self).with(|__s| __s.direction_)
        } == {
            other.with(|__s| __s.direction_)
        }) && ({
            (*self).with(|__s| __s.flexDirection_)
        } == {
            other.with(|__s| __s.flexDirection_)
        })) && ({
            (*self).with(|__s| __s.justifyContent_)
        } == {
            other.with(|__s| __s.justifyContent_)
        })) && ({
            (*self).with(|__s| __s.justifyItems_)
        } == {
            other.with(|__s| __s.justifyItems_)
        })) && ({
            (*self).with(|__s| __s.justifySelf_)
        } == {
            other.with(|__s| __s.justifySelf_)
        })) && ({
            (*self).with(|__s| __s.alignContent_)
        } == {
            other.with(|__s| __s.alignContent_)
        })) && ({
            (*self).with(|__s| __s.alignItems_)
        } == {
            other.with(|__s| __s.alignItems_)
        })) && ({
            (*self).with(|__s| __s.alignSelf_)
        } == {
            other.with(|__s| __s.alignSelf_)
        })) && ({
            (*self).with(|__s| __s.positionType_)
        } == {
            other.with(|__s| __s.positionType_)
        })) && ({ (*self).with(|__s| __s.flexWrap_) }
            == { other.with(|__s| __s.flexWrap_) }))
            && ({ (*self).with(|__s| __s.overflow_) } == {
                other.with(|__s| __s.overflow_)
            }))
            && ({ (*self).with(|__s| __s.display_) } == {
                other.with(|__s| __s.display_)
            }))
            && ({ (*self).with(|__s| __s.boxSizing_) } == {
                other.with(|__s| __s.boxSizing_)
            }))
            && ({
                let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!((*self), flex_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!(other, flex_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && ({
                let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> =
                    field_ptr!((*self), flexGrow_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!(other, flexGrow_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && ({
                let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> =
                    field_ptr!((*self), flexShrink_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> =
                    field_ptr!(other, flexShrink_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && ({
                let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> =
                    field_ptr!((*self), flexBasis_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> = field_ptr!(other, flexBasis_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), margin_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, margin_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), position_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, position_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), padding_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, padding_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), border_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, border_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), gap_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, gap_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_3(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), dimensions_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, dimensions_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), minDimensions_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, minDimensions_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!((*self), maxDimensions_).as_pointer();
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                    field_ref!(other, maxDimensions_).as_pointer();
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhsHandle: Ptr<facebook_yoga_StyleValueHandle> =
                    field_ptr!((*self), aspectRatio_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhsHandle: Ptr<facebook_yoga_StyleValueHandle> =
                    field_ptr!(other, aspectRatio_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::numbersEqual(_lhsHandle, _lhsPool, _rhsHandle, _rhsPool)
            }))
            && ({
                let _other: Ptr<facebook_yoga_GridStyleStorage> = field_ptr!(other, grid_);
                facebook_yoga_GridStyleStorageImpl::operator_eq(&field_ptr!((*self), grid_), _other)
            });
    }
    fn computeColumnGap(&self) -> facebook_yoga_StyleValueHandle {
        if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), gap_).decay().offset(
                    (({ to_underlying_49(facebook_yoga_Gutter_Column) }) as usize) as isize,
                ),
            )
        }) {
            return (*field_ref!((*self), gap_)
                .decay()
                .offset((({ to_underlying_49(facebook_yoga_Gutter_Column) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else {
            return (*field_ref!((*self), gap_)
                .decay()
                .offset((({ to_underlying_49(facebook_yoga_Gutter_All) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeRowGap(&self) -> facebook_yoga_StyleValueHandle {
        if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &field_ref!((*self), gap_)
                    .decay()
                    .offset((({ to_underlying_49(facebook_yoga_Gutter_Row) }) as usize) as isize),
            )
        }) {
            return (*field_ref!((*self), gap_)
                .decay()
                .offset((({ to_underlying_49(facebook_yoga_Gutter_Row) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else {
            return (*field_ref!((*self), gap_)
                .decay()
                .offset((({ to_underlying_49(facebook_yoga_Gutter_All) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeLeftEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if (layoutDirection == facebook_yoga_Direction_LTR)
            && ({
                facebook_yoga_StyleValueHandleImpl::isDefined(
                    &edges.decay().offset(
                        (({ to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as isize,
                    ),
                )
            })
        {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if (layoutDirection == facebook_yoga_Direction_RTL)
            && ({
                facebook_yoga_StyleValueHandleImpl::isDefined(
                    &edges
                        .decay()
                        .offset((({ to_underlying_46(facebook_yoga_Edge_End) }) as usize) as isize),
                )
            })
        {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_End) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Left) }) as usize) as isize),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Left) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges.decay().offset(
                    (({ to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as isize,
                ),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_All) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeTopEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
    ) -> facebook_yoga_StyleValueHandle {
        if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Top) }) as usize) as isize),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Top) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges.decay().offset(
                    (({ to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as isize,
                ),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_All) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeRightEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if (layoutDirection == facebook_yoga_Direction_LTR)
            && ({
                facebook_yoga_StyleValueHandleImpl::isDefined(
                    &edges
                        .decay()
                        .offset((({ to_underlying_46(facebook_yoga_Edge_End) }) as usize) as isize),
                )
            })
        {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_End) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if (layoutDirection == facebook_yoga_Direction_RTL)
            && ({
                facebook_yoga_StyleValueHandleImpl::isDefined(
                    &edges.decay().offset(
                        (({ to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as isize,
                    ),
                )
            })
        {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Start) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Right) }) as usize) as isize),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Right) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges.decay().offset(
                    (({ to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as isize,
                ),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Horizontal) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_All) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeBottomEdge(
        &self,
        edges: Ptr<Vec<facebook_yoga_StyleValueHandle>>,
    ) -> facebook_yoga_StyleValueHandle {
        if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges
                    .decay()
                    .offset((({ to_underlying_46(facebook_yoga_Edge_Bottom) }) as usize) as isize),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Bottom) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &edges.decay().offset(
                    (({ to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as isize,
                ),
            )
        }) {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_Vertical) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        } else {
            return (*edges
                .decay()
                .offset((({ to_underlying_46(facebook_yoga_Edge_All) }) as usize) as isize)
                .upgrade()
                .deref())
            .clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computePosition(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), position_).as_pointer();
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), position_).as_pointer();
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), position_).as_pointer();
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), position_).as_pointer();
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({
                    fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid physical edge"))
                });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    fn computeMargin(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), margin_).as_pointer();
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), margin_).as_pointer();
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), margin_).as_pointer();
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), margin_).as_pointer();
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({
                    fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid physical edge"))
                });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    fn computePadding(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), padding_).as_pointer();
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), padding_).as_pointer();
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), padding_).as_pointer();
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), padding_).as_pointer();
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({
                    fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid physical edge"))
                });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    fn computeBorder(
        &self,
        mut edge: facebook_yoga_PhysicalEdge,
        mut direction: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        switch!(match edge {
            __v if __v == facebook_yoga_PhysicalEdge_Left => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), border_).as_pointer();
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), border_).as_pointer();
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), border_).as_pointer();
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<Vec<facebook_yoga_StyleValueHandle>> =
                        field_ref!((*self), border_).as_pointer();
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({
                    fatalWithMessage_103(Ptr::<i8>::from_string_literal(b"Invalid physical edge"))
                });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    fn resolve(
        &self,
        handle: facebook_yoga_StyleValueHandle,
        mut referenceLength: f32,
    ) -> facebook_yoga_FloatOptional {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(handle));
        if ({ facebook_yoga_StyleValueHandleImpl::isPoint(&handle.as_pointer()) }) {
            return facebook_yoga_FloatOptional::new_1({
                ({
                    facebook_yoga_StyleValuePoolImpl::getStoredValue(
                        &field_ptr!((*self), pool_),
                        (*handle.borrow()).clone(),
                    )
                })
            });
        }
        if ({ facebook_yoga_StyleValueHandleImpl::isPercent(&handle.as_pointer()) }) {
            return facebook_yoga_FloatOptional::new_1({
                ((({
                    facebook_yoga_StyleValuePoolImpl::getStoredValue(
                        &field_ptr!((*self), pool_),
                        (*handle.borrow()).clone(),
                    )
                }) * referenceLength)
                    * 0.01_f32)
            });
        }
        return <facebook_yoga_FloatOptional>::default();
    }
    fn copy_assign(&self, _a0: Ptr<facebook_yoga_Style>) -> Ptr<facebook_yoga_Style> {
        field!((*self), direction_).write({ _a0.with(|__s| __s.direction_) });
        field!((*self), flexDirection_).write({ _a0.with(|__s| __s.flexDirection_) });
        field!((*self), justifyContent_).write({ _a0.with(|__s| __s.justifyContent_) });
        field!((*self), justifyItems_).write({ _a0.with(|__s| __s.justifyItems_) });
        field!((*self), justifySelf_).write({ _a0.with(|__s| __s.justifySelf_) });
        field!((*self), alignContent_).write({ _a0.with(|__s| __s.alignContent_) });
        field!((*self), alignItems_).write({ _a0.with(|__s| __s.alignItems_) });
        field!((*self), alignSelf_).write({ _a0.with(|__s| __s.alignSelf_) });
        field!((*self), positionType_).write({ _a0.with(|__s| __s.positionType_) });
        field!((*self), flexWrap_).write({ _a0.with(|__s| __s.flexWrap_) });
        field!((*self), overflow_).write({ _a0.with(|__s| __s.overflow_) });
        field!((*self), display_).write({ _a0.with(|__s| __s.display_) });
        field!((*self), boxSizing_).write({ _a0.with(|__s| __s.boxSizing_) });
        field!((*self), flex_).write({ _a0.with(|__s| __s.flex_.clone()) });
        field!((*self), flexGrow_).write({ _a0.with(|__s| __s.flexGrow_.clone()) });
        field!((*self), flexShrink_).write({ _a0.with(|__s| __s.flexShrink_.clone()) });
        field!((*self), flexBasis_).write({ _a0.with(|__s| __s.flexBasis_.clone()) });
        field!((*self), margin_).write({ _a0.with(|__s| __s.margin_.clone()) });
        field!((*self), position_).write({ _a0.with(|__s| __s.position_.clone()) });
        field!((*self), padding_).write({ _a0.with(|__s| __s.padding_.clone()) });
        field!((*self), border_).write({ _a0.with(|__s| __s.border_.clone()) });
        field!((*self), gap_).write({ _a0.with(|__s| __s.gap_.clone()) });
        field!((*self), dimensions_).write({ _a0.with(|__s| __s.dimensions_.clone()) });
        field!((*self), minDimensions_).write({ _a0.with(|__s| __s.minDimensions_.clone()) });
        field!((*self), maxDimensions_).write({ _a0.with(|__s| __s.maxDimensions_.clone()) });
        field!((*self), aspectRatio_).write({ _a0.with(|__s| __s.aspectRatio_.clone()) });
        ({
            let _other: Ptr<facebook_yoga_GridStyleStorage> = field_ptr!(_a0, grid_);
            facebook_yoga_GridStyleStorageImpl::copy_assign(&field_ptr!((*self), grid_), _other)
        });
        ({
            let _arg0: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(_a0, pool_);
            facebook_yoga_StyleValuePoolImpl::copy_assign(&field_ptr!((*self), pool_), _arg0)
        });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<facebook_yoga_Style>) -> Ptr<facebook_yoga_Style> {
        field!((*self), direction_).write({ { (*_a0.upgrade().deref()).direction_ } });
        field!((*self), flexDirection_).write({ { (*_a0.upgrade().deref()).flexDirection_ } });
        field!((*self), justifyContent_).write({ { (*_a0.upgrade().deref()).justifyContent_ } });
        field!((*self), justifyItems_).write({ { (*_a0.upgrade().deref()).justifyItems_ } });
        field!((*self), justifySelf_).write({ { (*_a0.upgrade().deref()).justifySelf_ } });
        field!((*self), alignContent_).write({ { (*_a0.upgrade().deref()).alignContent_ } });
        field!((*self), alignItems_).write({ { (*_a0.upgrade().deref()).alignItems_ } });
        field!((*self), alignSelf_).write({ { (*_a0.upgrade().deref()).alignSelf_ } });
        field!((*self), positionType_).write({ { (*_a0.upgrade().deref()).positionType_ } });
        field!((*self), flexWrap_).write({ { (*_a0.upgrade().deref()).flexWrap_ } });
        field!((*self), overflow_).write({ { (*_a0.upgrade().deref()).overflow_ } });
        field!((*self), display_).write({ { (*_a0.upgrade().deref()).display_ } });
        field!((*self), boxSizing_).write({ { (*_a0.upgrade().deref()).boxSizing_ } });
        field!((*self), flex_).write({ { (*_a0.upgrade().deref()).flex_.clone() } });
        field!((*self), flexGrow_).write({ { (*_a0.upgrade().deref()).flexGrow_.clone() } });
        field!((*self), flexShrink_).write({ { (*_a0.upgrade().deref()).flexShrink_.clone() } });
        field!((*self), flexBasis_).write({ { (*_a0.upgrade().deref()).flexBasis_.clone() } });
        field!((*self), margin_).write({ { (*_a0.upgrade().deref()).margin_.clone() } });
        field!((*self), position_).write({ { (*_a0.upgrade().deref()).position_.clone() } });
        field!((*self), padding_).write({ { (*_a0.upgrade().deref()).padding_.clone() } });
        field!((*self), border_).write({ { (*_a0.upgrade().deref()).border_.clone() } });
        field!((*self), gap_).write({ { (*_a0.upgrade().deref()).gap_.clone() } });
        field!((*self), dimensions_).write({ { (*_a0.upgrade().deref()).dimensions_.clone() } });
        field!((*self), minDimensions_)
            .write({ { (*_a0.upgrade().deref()).minDimensions_.clone() } });
        field!((*self), maxDimensions_)
            .write({ { (*_a0.upgrade().deref()).maxDimensions_.clone() } });
        field!((*self), aspectRatio_).write({ { (*_a0.upgrade().deref()).aspectRatio_.clone() } });
        ({
            let _arg0: Ptr<facebook_yoga_GridStyleStorage> = field_ptr!(_a0, grid_);
            facebook_yoga_GridStyleStorageImpl::move_assign(&field_ptr!((*self), grid_), _arg0)
        });
        ({
            let _arg0: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(_a0, pool_);
            facebook_yoga_StyleValuePoolImpl::move_assign(&field_ptr!((*self), pool_), _arg0)
        });
        return (*self).clone();
    }
}
pub trait facebook_yoga_StyleLengthImpl {
    fn isAuto(&self) -> bool;
    fn isUndefined(&self) -> bool;
    fn isPoints(&self) -> bool;
    fn isPercent(&self) -> bool;
    fn isDefined(&self) -> bool;
    fn value(&self) -> facebook_yoga_FloatOptional;
    fn resolve(&self, referenceLength: f32) -> facebook_yoga_FloatOptional;
    fn to_YGValue(&self) -> YGValue;
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleLength>) -> bool;
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleLength>) -> bool;
}
impl facebook_yoga_StyleLengthImpl for Ptr<facebook_yoga_StyleLength> {
    fn isAuto(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Auto);
    }
    fn isUndefined(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Undefined);
    }
    fn isPoints(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Point);
    }
    fn isPercent(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Percent);
    }
    fn isDefined(&self) -> bool {
        return !({ facebook_yoga_StyleLengthImpl::isUndefined(self) });
    }
    fn value(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.value_.clone());
    }
    fn resolve(&self, mut referenceLength: f32) -> facebook_yoga_FloatOptional {
        'switch: {
            match { (*self).with(|__s| __s.unit_) } {
                __v if __v == facebook_yoga_Unit_Point => {
                    return (*self).with(|__s| __s.value_.clone());
                }
                __v if __v == facebook_yoga_Unit_Percent => {
                    return facebook_yoga_FloatOptional::new_1({
                        ((({
                            facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!((*self), value_))
                        }) * referenceLength)
                            * 0.01_f32)
                    });
                }
                _ => {
                    return <facebook_yoga_FloatOptional>::default();
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
    fn to_YGValue(&self) -> YGValue {
        return YGValue {
            value: ({ facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!((*self), value_)) }),
            unit: ({ unscopedEnum_164((*self).with(|__s| __s.unit_)) }),
        };
    }
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleLength>) -> bool {
        return ({
            let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
            let _rhs: facebook_yoga_FloatOptional = rhs.with(|__s| __s.value_.clone());
            operator_eq_114(_lhs, _rhs)
        }) && ({ (*self).with(|__s| __s.unit_) } == { rhs.with(|__s| __s.unit_) });
    }
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleLength>) -> bool {
        return ({ (*self).with(|__s| __s.unit_) } == { other.with(|__s| __s.unit_) })
            && ({
                let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
                let _rhs: facebook_yoga_FloatOptional = other.with(|__s| __s.value_.clone());
                inexactEquals_122(_lhs, _rhs)
            });
    }
}
pub trait facebook_yoga_StyleSizeLengthImpl {
    fn isAuto(&self) -> bool;
    fn isMaxContent(&self) -> bool;
    fn isFitContent(&self) -> bool;
    fn isStretch(&self) -> bool;
    fn isUndefined(&self) -> bool;
    fn isDefined(&self) -> bool;
    fn isPoints(&self) -> bool;
    fn isPercent(&self) -> bool;
    fn value(&self) -> facebook_yoga_FloatOptional;
    fn resolve(&self, referenceLength: f32) -> facebook_yoga_FloatOptional;
    fn to_YGValue(&self) -> YGValue;
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleSizeLength>) -> bool;
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleSizeLength>) -> bool;
}
impl facebook_yoga_StyleSizeLengthImpl for Ptr<facebook_yoga_StyleSizeLength> {
    fn isAuto(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Auto);
    }
    fn isMaxContent(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_MaxContent);
    }
    fn isFitContent(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_FitContent);
    }
    fn isStretch(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Stretch);
    }
    fn isUndefined(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Undefined);
    }
    fn isDefined(&self) -> bool {
        return !({ facebook_yoga_StyleSizeLengthImpl::isUndefined(self) });
    }
    fn isPoints(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Point);
    }
    fn isPercent(&self) -> bool {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Percent);
    }
    fn value(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.value_.clone());
    }
    fn resolve(&self, mut referenceLength: f32) -> facebook_yoga_FloatOptional {
        'switch: {
            match { (*self).with(|__s| __s.unit_) } {
                __v if __v == facebook_yoga_Unit_Point => {
                    return (*self).with(|__s| __s.value_.clone());
                }
                __v if __v == facebook_yoga_Unit_Percent => {
                    return facebook_yoga_FloatOptional::new_1({
                        ((({
                            facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!((*self), value_))
                        }) * referenceLength)
                            * 0.01_f32)
                    });
                }
                _ => {
                    return <facebook_yoga_FloatOptional>::default();
                }
            }
        };
        panic!("ub: non-void function does not return a value")
    }
    fn to_YGValue(&self) -> YGValue {
        return YGValue {
            value: ({ facebook_yoga_FloatOptionalImpl::unwrap(&field_ptr!((*self), value_)) }),
            unit: ({ unscopedEnum_164((*self).with(|__s| __s.unit_)) }),
        };
    }
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleSizeLength>) -> bool {
        return ({
            let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
            let _rhs: facebook_yoga_FloatOptional = rhs.with(|__s| __s.value_.clone());
            operator_eq_114(_lhs, _rhs)
        }) && ({ (*self).with(|__s| __s.unit_) } == { rhs.with(|__s| __s.unit_) });
    }
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleSizeLength>) -> bool {
        return ({ (*self).with(|__s| __s.unit_) } == { other.with(|__s| __s.unit_) })
            && ({
                let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
                let _rhs: facebook_yoga_FloatOptional = other.with(|__s| __s.value_.clone());
                inexactEquals_122(_lhs, _rhs)
            });
    }
}
pub trait facebook_yoga_StyleValueHandleImpl {
    fn isUndefined(&self) -> bool;
    fn isDefined(&self) -> bool;
    fn isAuto(&self) -> bool;
    fn isPercent(&self) -> bool;
    fn isPoint(&self) -> bool;
    fn isKeyword(&self, keyword: facebook_yoga_StyleValueHandle_Keyword) -> bool;
    fn type_(&self) -> facebook_yoga_StyleValueHandle_Type;
    fn setType(&self, handleType: facebook_yoga_StyleValueHandle_Type);
    fn value(&self) -> u16;
    fn setValue(&self, value: u16);
    fn isValueIndexed(&self) -> bool;
    fn setValueIsIndexed(&self);
}
impl facebook_yoga_StyleValueHandleImpl for Ptr<facebook_yoga_StyleValueHandle> {
    fn isUndefined(&self) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Undefined);
    }
    fn isDefined(&self) -> bool {
        return !({ facebook_yoga_StyleValueHandleImpl::isUndefined(self) });
    }
    fn isAuto(&self) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Auto);
    }
    fn isPercent(&self) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Percent);
    }
    fn isPoint(&self) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Point);
    }
    fn isKeyword(&self, mut keyword: facebook_yoga_StyleValueHandle_Keyword) -> bool {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Keyword)
            && ((({ facebook_yoga_StyleValueHandleImpl::value(self) }) as i32)
                == ((keyword as u16) as i32));
    }
    fn type_(&self) -> facebook_yoga_StyleValueHandle_Type {
        return ((((*self).with(|__s| __s.repr_) as i32)
            & (kHandleTypeMask_174.with(|rc| *rc.borrow()) as i32))
            as facebook_yoga_StyleValueHandle_Type);
    }
    fn setType(&self, mut handleType: facebook_yoga_StyleValueHandle_Type) {
        field!((*self), repr_).write({
            (((*self).with(|__s| __s.repr_) as i32)
                & (!(kHandleTypeMask_174.with(|rc| *rc.borrow()) as i32))) as u16
        });
        field!((*self), repr_).write({
            (((*self).with(|__s| __s.repr_) as i32) | ((handleType as u8) as i32)) as u16
        });
    }
    fn value(&self) -> u16 {
        return ((((*self).with(|__s| __s.repr_) as i32) >> 4) as u16);
    }
    fn setValue(&self, mut value: u16) {
        field!((*self), repr_).write({
            (((*self).with(|__s| __s.repr_) as i32)
                & (!(kHandleValueMask_176.with(|rc| *rc.borrow()) as i32))) as u16
        });
        field!((*self), repr_)
            .write({ (((*self).with(|__s| __s.repr_) as i32) | ((value as i32) << 4)) as u16 });
    }
    fn isValueIndexed(&self) -> bool {
        return ((((*self).with(|__s| __s.repr_) as i32)
            & (kHandleIndexedMask_175.with(|rc| *rc.borrow()) as i32))
            != 0);
    }
    fn setValueIsIndexed(&self) {
        field!((*self), repr_).write({
            (((*self).with(|__s| __s.repr_) as i32)
                | (kHandleIndexedMask_175.with(|rc| *rc.borrow()) as i32)) as u16
        });
    }
}
pub trait facebook_yoga_StyleValuePoolImpl {
    fn store_1(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        length: facebook_yoga_StyleLength,
    );
    fn store_2(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        sizeValue: facebook_yoga_StyleSizeLength,
    );
    fn store_3(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        number: facebook_yoga_FloatOptional,
    );
    fn getLength(&self, handle: facebook_yoga_StyleValueHandle) -> facebook_yoga_StyleLength;
    fn getSize(&self, handle: facebook_yoga_StyleValueHandle) -> facebook_yoga_StyleSizeLength;
    fn getNumber(&self, handle: facebook_yoga_StyleValueHandle) -> facebook_yoga_FloatOptional;
    fn getStoredValue(&self, handle: facebook_yoga_StyleValueHandle) -> f32;
    fn storeValue(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        value: f32,
        type_: facebook_yoga_StyleValueHandle_Type,
    );
    fn storeKeyword(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        keyword: facebook_yoga_StyleValueHandle_Keyword,
    );
    fn copy_assign(
        &self,
        _a0: Ptr<facebook_yoga_StyleValuePool>,
    ) -> Ptr<facebook_yoga_StyleValuePool>;
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_StyleValuePool>,
    ) -> Ptr<facebook_yoga_StyleValuePool>;
}
impl facebook_yoga_StyleValuePoolImpl for Ptr<facebook_yoga_StyleValuePool> {
    fn store_1(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        length: facebook_yoga_StyleLength,
    ) {
        let length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(length));
        if ({ facebook_yoga_StyleLengthImpl::isUndefined(&length.as_pointer()) }) {
            ({
                facebook_yoga_StyleValueHandleImpl::setType(
                    &handle,
                    facebook_yoga_StyleValueHandle_Type_Undefined,
                )
            });
        } else if ({ facebook_yoga_StyleLengthImpl::isAuto(&length.as_pointer()) }) {
            ({
                facebook_yoga_StyleValueHandleImpl::setType(
                    &handle,
                    facebook_yoga_StyleValueHandle_Type_Auto,
                )
            });
        } else {
            let mut type_: facebook_yoga_StyleValueHandle_Type =
                if ({ facebook_yoga_StyleLengthImpl::isPoints(&length.as_pointer()) }) {
                    facebook_yoga_StyleValueHandle_Type_Point
                } else {
                    facebook_yoga_StyleValueHandle_Type_Percent
                };
            ({
                let _handle: Ptr<facebook_yoga_StyleValueHandle> = (handle).clone();
                let _value: f32 = ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({ facebook_yoga_StyleLengthImpl::value(&length.as_pointer()) }),
                        ))
                        .as_pointer(),
                    )
                });
                let _type_: facebook_yoga_StyleValueHandle_Type = type_;
                facebook_yoga_StyleValuePoolImpl::storeValue(self, _handle, _value, _type_)
            });
        }
    }
    fn store_2(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        sizeValue: facebook_yoga_StyleSizeLength,
    ) {
        let sizeValue: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(sizeValue));
        if ({ facebook_yoga_StyleSizeLengthImpl::isUndefined(&sizeValue.as_pointer()) }) {
            ({
                facebook_yoga_StyleValueHandleImpl::setType(
                    &handle,
                    facebook_yoga_StyleValueHandle_Type_Undefined,
                )
            });
        } else if ({ facebook_yoga_StyleSizeLengthImpl::isAuto(&sizeValue.as_pointer()) }) {
            ({
                facebook_yoga_StyleValueHandleImpl::setType(
                    &handle,
                    facebook_yoga_StyleValueHandle_Type_Auto,
                )
            });
        } else if ({ facebook_yoga_StyleSizeLengthImpl::isMaxContent(&sizeValue.as_pointer()) }) {
            ({
                facebook_yoga_StyleValuePoolImpl::storeKeyword(
                    self,
                    (handle).clone(),
                    facebook_yoga_StyleValueHandle_Keyword_MaxContent,
                )
            });
        } else if ({ facebook_yoga_StyleSizeLengthImpl::isStretch(&sizeValue.as_pointer()) }) {
            ({
                facebook_yoga_StyleValuePoolImpl::storeKeyword(
                    self,
                    (handle).clone(),
                    facebook_yoga_StyleValueHandle_Keyword_Stretch,
                )
            });
        } else if ({ facebook_yoga_StyleSizeLengthImpl::isFitContent(&sizeValue.as_pointer()) }) {
            ({
                facebook_yoga_StyleValuePoolImpl::storeKeyword(
                    self,
                    (handle).clone(),
                    facebook_yoga_StyleValueHandle_Keyword_FitContent,
                )
            });
        } else {
            let mut type_: facebook_yoga_StyleValueHandle_Type =
                if ({ facebook_yoga_StyleSizeLengthImpl::isPoints(&sizeValue.as_pointer()) }) {
                    facebook_yoga_StyleValueHandle_Type_Point
                } else {
                    facebook_yoga_StyleValueHandle_Type_Percent
                };
            ({
                let _handle: Ptr<facebook_yoga_StyleValueHandle> = (handle).clone();
                let _value: f32 = ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({ facebook_yoga_StyleSizeLengthImpl::value(&sizeValue.as_pointer()) }),
                        ))
                        .as_pointer(),
                    )
                });
                let _type_: facebook_yoga_StyleValueHandle_Type = type_;
                facebook_yoga_StyleValuePoolImpl::storeValue(self, _handle, _value, _type_)
            });
        }
    }
    fn store_3(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        number: facebook_yoga_FloatOptional,
    ) {
        let number: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(number));
        if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&number.as_pointer()) }) {
            ({
                facebook_yoga_StyleValueHandleImpl::setType(
                    &handle,
                    facebook_yoga_StyleValueHandle_Type_Undefined,
                )
            });
        } else {
            ({
                let _handle: Ptr<facebook_yoga_StyleValueHandle> = (handle).clone();
                let _value: f32 =
                    ({ facebook_yoga_FloatOptionalImpl::unwrap(&number.as_pointer()) });
                facebook_yoga_StyleValuePoolImpl::storeValue(
                    self,
                    _handle,
                    _value,
                    facebook_yoga_StyleValueHandle_Type_Number,
                )
            });
        }
    }
    fn getLength(&self, handle: facebook_yoga_StyleValueHandle) -> facebook_yoga_StyleLength {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(handle));
        if ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&handle.as_pointer()) }) {
            return ({ facebook_yoga_StyleLength::undefined() });
        } else if ({ facebook_yoga_StyleValueHandleImpl::isAuto(&handle.as_pointer()) }) {
            return ({ facebook_yoga_StyleLength::ofAuto() });
        } else {
            assert!(
                (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                    == facebook_yoga_StyleValueHandle_Type_Point)
                    || (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                        == facebook_yoga_StyleValueHandle_Type_Percent)
            );
            let mut value: f32 = if ({
                facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer())
            }) {
                {
                    let mut __bytes = vec![0_u8; <u32 as ByteRepr>::byte_size()];
                    ({
                        facebook_yoga_SmallValueBuffer_4_Impl::get32(
                            &field_ptr!((*self), buffer_),
                            ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                        )
                    })
                    .to_bytes(&mut __bytes);
                    <f32>::from_bytes(&__bytes)
                }
            } else {
                ({
                    facebook_yoga_StyleValuePool::unpackInlineInteger(
                        ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                    )
                })
            };
            return if (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer()) })
                == facebook_yoga_StyleValueHandle_Type_Point)
            {
                ({ facebook_yoga_StyleLength::points(value) })
            } else {
                ({ facebook_yoga_StyleLength::percent(value) })
            };
        }
        panic!("ub: non-void function does not return a value")
    }
    fn getSize(&self, handle: facebook_yoga_StyleValueHandle) -> facebook_yoga_StyleSizeLength {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(handle));
        if ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&handle.as_pointer()) }) {
            return ({ facebook_yoga_StyleSizeLength::undefined() });
        } else if ({ facebook_yoga_StyleValueHandleImpl::isAuto(&handle.as_pointer()) }) {
            return ({ facebook_yoga_StyleSizeLength::ofAuto() });
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isKeyword(
                &handle.as_pointer(),
                facebook_yoga_StyleValueHandle_Keyword_MaxContent,
            )
        }) {
            return ({ facebook_yoga_StyleSizeLength::ofMaxContent() });
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isKeyword(
                &handle.as_pointer(),
                facebook_yoga_StyleValueHandle_Keyword_FitContent,
            )
        }) {
            return ({ facebook_yoga_StyleSizeLength::ofFitContent() });
        } else if ({
            facebook_yoga_StyleValueHandleImpl::isKeyword(
                &handle.as_pointer(),
                facebook_yoga_StyleValueHandle_Keyword_Stretch,
            )
        }) {
            return ({ facebook_yoga_StyleSizeLength::ofStretch() });
        } else {
            assert!(
                (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                    == facebook_yoga_StyleValueHandle_Type_Point)
                    || (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                        == facebook_yoga_StyleValueHandle_Type_Percent)
            );
            let mut value: f32 = if ({
                facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer())
            }) {
                {
                    let mut __bytes = vec![0_u8; <u32 as ByteRepr>::byte_size()];
                    ({
                        facebook_yoga_SmallValueBuffer_4_Impl::get32(
                            &field_ptr!((*self), buffer_),
                            ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                        )
                    })
                    .to_bytes(&mut __bytes);
                    <f32>::from_bytes(&__bytes)
                }
            } else {
                ({
                    facebook_yoga_StyleValuePool::unpackInlineInteger(
                        ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                    )
                })
            };
            return if (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer()) })
                == facebook_yoga_StyleValueHandle_Type_Point)
            {
                ({ facebook_yoga_StyleSizeLength::points(value) })
            } else {
                ({ facebook_yoga_StyleSizeLength::percent(value) })
            };
        }
        panic!("ub: non-void function does not return a value")
    }
    fn getNumber(&self, handle: facebook_yoga_StyleValueHandle) -> facebook_yoga_FloatOptional {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(handle));
        if ({ facebook_yoga_StyleValueHandleImpl::isUndefined(&handle.as_pointer()) }) {
            return <facebook_yoga_FloatOptional>::default();
        } else {
            assert!(
                (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                    == facebook_yoga_StyleValueHandle_Type_Number)
            );
            let mut value: f32 = if ({
                facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer())
            }) {
                {
                    let mut __bytes = vec![0_u8; <u32 as ByteRepr>::byte_size()];
                    ({
                        facebook_yoga_SmallValueBuffer_4_Impl::get32(
                            &field_ptr!((*self), buffer_),
                            ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                        )
                    })
                    .to_bytes(&mut __bytes);
                    <f32>::from_bytes(&__bytes)
                }
            } else {
                ({
                    facebook_yoga_StyleValuePool::unpackInlineInteger(
                        ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                    )
                })
            };
            return facebook_yoga_FloatOptional::new_1({ value });
        }
        panic!("ub: non-void function does not return a value")
    }
    fn getStoredValue(&self, handle: facebook_yoga_StyleValueHandle) -> f32 {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(handle));
        assert!(
            ((({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                == facebook_yoga_StyleValueHandle_Type_Point)
                || (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                    == facebook_yoga_StyleValueHandle_Type_Percent))
                || (({ facebook_yoga_StyleValueHandleImpl::type_(&handle.as_pointer(),) })
                    == facebook_yoga_StyleValueHandle_Type_Number)
        );
        return if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer()) }) {
            {
                let mut __bytes = vec![0_u8; <u32 as ByteRepr>::byte_size()];
                ({
                    facebook_yoga_SmallValueBuffer_4_Impl::get32(
                        &field_ptr!((*self), buffer_),
                        ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                    )
                })
                .to_bytes(&mut __bytes);
                <f32>::from_bytes(&__bytes)
            }
        } else {
            ({
                facebook_yoga_StyleValuePool::unpackInlineInteger(
                    ({ facebook_yoga_StyleValueHandleImpl::value(&handle.as_pointer()) }),
                )
            })
        };
    }
    fn storeValue(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        mut value: f32,
        mut type_: facebook_yoga_StyleValueHandle_Type,
    ) {
        ({
            let _handleType: facebook_yoga_StyleValueHandle_Type = type_;
            facebook_yoga_StyleValueHandleImpl::setType(&handle, _handleType)
        });
        if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle) }) {
            let mut newIndex: u16 = ({
                facebook_yoga_SmallValueBuffer_4_Impl::replace_6(
                    &field_ptr!((*self), buffer_),
                    ({ facebook_yoga_StyleValueHandleImpl::value(&handle) }),
                    {
                        let mut __bytes = vec![0_u8; <f32 as ByteRepr>::byte_size()];
                        value.to_bytes(&mut __bytes);
                        <u32>::from_bytes(&__bytes)
                    },
                )
            });
            ({
                let _value: u16 = newIndex;
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        } else if ({ facebook_yoga_StyleValuePool::isIntegerPackable(value) }) {
            ({
                let _value: u16 = ({ facebook_yoga_StyleValuePool::packInlineInteger(value) });
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        } else {
            let mut newIndex: u16 = ({
                facebook_yoga_SmallValueBuffer_4_Impl::push_4(&field_ptr!((*self), buffer_), {
                    let mut __bytes = vec![0_u8; <f32 as ByteRepr>::byte_size()];
                    value.to_bytes(&mut __bytes);
                    <u32>::from_bytes(&__bytes)
                })
            });
            ({
                let _value: u16 = newIndex;
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
            ({ facebook_yoga_StyleValueHandleImpl::setValueIsIndexed(&handle) });
        }
    }
    fn storeKeyword(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        mut keyword: facebook_yoga_StyleValueHandle_Keyword,
    ) {
        ({
            facebook_yoga_StyleValueHandleImpl::setType(
                &handle,
                facebook_yoga_StyleValueHandle_Type_Keyword,
            )
        });
        if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle) }) {
            let mut newIndex: u16 = ({
                facebook_yoga_SmallValueBuffer_4_Impl::replace_6(
                    &field_ptr!((*self), buffer_),
                    ({ facebook_yoga_StyleValueHandleImpl::value(&handle) }),
                    (keyword as u32),
                )
            });
            ({
                let _value: u16 = newIndex;
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        } else {
            ({
                let _value: u16 = (keyword as u16);
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        }
    }
    fn copy_assign(
        &self,
        _a0: Ptr<facebook_yoga_StyleValuePool>,
    ) -> Ptr<facebook_yoga_StyleValuePool> {
        ({
            let _other: Ptr<facebook_yoga_SmallValueBuffer_4_> = field_ptr!(_a0, buffer_);
            facebook_yoga_SmallValueBuffer_4_Impl::copy_assign(
                &field_ptr!((*self), buffer_),
                _other,
            )
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_StyleValuePool>,
    ) -> Ptr<facebook_yoga_StyleValuePool> {
        ({
            let _other: Ptr<facebook_yoga_SmallValueBuffer_4_> = field_ptr!(_a0, buffer_);
            facebook_yoga_SmallValueBuffer_4_Impl::move_assign(
                &field_ptr!((*self), buffer_),
                _other,
            )
        });
        return (*self).clone();
    }
}
pub trait facebook_yoga__anonymous_namespace__NodeImpl {
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga__anonymous_namespace__Node>,
    ) -> Ptr<facebook_yoga__anonymous_namespace__Node>;
}
impl facebook_yoga__anonymous_namespace__NodeImpl
    for Ptr<facebook_yoga__anonymous_namespace__Node>
{
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga__anonymous_namespace__Node>,
    ) -> Ptr<facebook_yoga__anonymous_namespace__Node> {
        field!((*self), subscriber)
            .with_mut(|__b0| (*__b0) = (*_a0.upgrade().deref()).subscriber.move_from());
        field!((*self), next).write({ { (*_a0.upgrade().deref()).next.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = YGUndefined_23.with(|_| ());
    let _ = defaultConfig__71.with(|_| ());
    let _ = MaxCachedMeasurements_123.with(|_| ());
    let _ = kHandleTypeMask_174.with(|_| ());
    let _ = kHandleIndexedMask_175.with(|_| ());
    let _ = kHandleValueMask_176.with(|_| ());
    let _ = DefaultFlexGrow_177.with(|_| ());
    let _ = DefaultFlexShrink_178.with(|_| ());
    let _ = WebDefaultFlexShrink_179.with(|_| ());
    let _ = YGValueZero_2.with(|_| ());
    let _ = YGValueUndefined_1.with(|_| ());
    let _ = YGValueAuto_0.with(|_| ());
    let _ = gCurrentGenerationCount_439.with(|_| ());
    let _ = subscribers_476.with(|_| ());
}
