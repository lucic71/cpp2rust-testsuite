extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type YGAlign = ();
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
pub type YGBoxSizing = ();
pub const YGBoxSizing_YGBoxSizingBorderBox: YGBoxSizing = 0;
pub const YGBoxSizing_YGBoxSizingContentBox: YGBoxSizing = 1;
pub type YGDimension = ();
pub const YGDimension_YGDimensionWidth: YGDimension = 0;
pub const YGDimension_YGDimensionHeight: YGDimension = 1;
pub type YGDirection = ();
pub const YGDirection_YGDirectionInherit: YGDirection = 0;
pub const YGDirection_YGDirectionLTR: YGDirection = 1;
pub const YGDirection_YGDirectionRTL: YGDirection = 2;
pub type YGDisplay = ();
pub const YGDisplay_YGDisplayFlex: YGDisplay = 0;
pub const YGDisplay_YGDisplayNone: YGDisplay = 1;
pub const YGDisplay_YGDisplayContents: YGDisplay = 2;
pub const YGDisplay_YGDisplayGrid: YGDisplay = 3;
pub type YGEdge = ();
pub const YGEdge_YGEdgeLeft: YGEdge = 0;
pub const YGEdge_YGEdgeTop: YGEdge = 1;
pub const YGEdge_YGEdgeRight: YGEdge = 2;
pub const YGEdge_YGEdgeBottom: YGEdge = 3;
pub const YGEdge_YGEdgeStart: YGEdge = 4;
pub const YGEdge_YGEdgeEnd: YGEdge = 5;
pub const YGEdge_YGEdgeHorizontal: YGEdge = 6;
pub const YGEdge_YGEdgeVertical: YGEdge = 7;
pub const YGEdge_YGEdgeAll: YGEdge = 8;
pub type YGErrata = ();
pub const YGErrata_YGErrataNone: YGErrata = 0;
pub const YGErrata_YGErrataStretchFlexBasis: YGErrata = 1;
pub const YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding: YGErrata = 2;
pub const YGErrata_YGErrataAbsolutePercentAgainstInnerSize: YGErrata = 4;
pub const YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto: YGErrata = 8;
pub const YGErrata_YGErrataFlexFirstPassUsesRunningTotals: YGErrata = 16;
pub const YGErrata_YGErrataAll: YGErrata = 2147483647;
pub const YGErrata_YGErrataClassic: YGErrata = 2147483646;
pub type YGExperimentalFeature = ();
pub const YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis: YGExperimentalFeature = 0;
pub const YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent: YGExperimentalFeature =
    1;
pub type YGFlexDirection = ();
pub const YGFlexDirection_YGFlexDirectionColumn: YGFlexDirection = 0;
pub const YGFlexDirection_YGFlexDirectionColumnReverse: YGFlexDirection = 1;
pub const YGFlexDirection_YGFlexDirectionRow: YGFlexDirection = 2;
pub const YGFlexDirection_YGFlexDirectionRowReverse: YGFlexDirection = 3;
pub type YGGridTrackType = ();
pub const YGGridTrackType_YGGridTrackTypeAuto: YGGridTrackType = 0;
pub const YGGridTrackType_YGGridTrackTypePoints: YGGridTrackType = 1;
pub const YGGridTrackType_YGGridTrackTypePercent: YGGridTrackType = 2;
pub const YGGridTrackType_YGGridTrackTypeFr: YGGridTrackType = 3;
pub const YGGridTrackType_YGGridTrackTypeMinmax: YGGridTrackType = 4;
pub type YGGutter = ();
pub const YGGutter_YGGutterColumn: YGGutter = 0;
pub const YGGutter_YGGutterRow: YGGutter = 1;
pub const YGGutter_YGGutterAll: YGGutter = 2;
pub type YGJustify = ();
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
pub type YGLogLevel = ();
pub const YGLogLevel_YGLogLevelError: YGLogLevel = 0;
pub const YGLogLevel_YGLogLevelWarn: YGLogLevel = 1;
pub const YGLogLevel_YGLogLevelInfo: YGLogLevel = 2;
pub const YGLogLevel_YGLogLevelDebug: YGLogLevel = 3;
pub const YGLogLevel_YGLogLevelVerbose: YGLogLevel = 4;
pub const YGLogLevel_YGLogLevelFatal: YGLogLevel = 5;
pub type YGMeasureMode = ();
pub const YGMeasureMode_YGMeasureModeUndefined: YGMeasureMode = 0;
pub const YGMeasureMode_YGMeasureModeExactly: YGMeasureMode = 1;
pub const YGMeasureMode_YGMeasureModeAtMost: YGMeasureMode = 2;
pub type YGNodeType = ();
pub const YGNodeType_YGNodeTypeDefault: YGNodeType = 0;
pub const YGNodeType_YGNodeTypeText: YGNodeType = 1;
pub type YGOverflow = ();
pub const YGOverflow_YGOverflowVisible: YGOverflow = 0;
pub const YGOverflow_YGOverflowHidden: YGOverflow = 1;
pub const YGOverflow_YGOverflowScroll: YGOverflow = 2;
pub type YGPositionType = ();
pub const YGPositionType_YGPositionTypeStatic: YGPositionType = 0;
pub const YGPositionType_YGPositionTypeRelative: YGPositionType = 1;
pub const YGPositionType_YGPositionTypeAbsolute: YGPositionType = 2;
pub type YGUnit = ();
pub const YGUnit_YGUnitUndefined: YGUnit = 0;
pub const YGUnit_YGUnitPoint: YGUnit = 1;
pub const YGUnit_YGUnitPercent: YGUnit = 2;
pub const YGUnit_YGUnitAuto: YGUnit = 3;
pub const YGUnit_YGUnitMaxContent: YGUnit = 4;
pub const YGUnit_YGUnitFitContent: YGUnit = 5;
pub const YGUnit_YGUnitStretch: YGUnit = 6;
pub type YGWrap = ();
pub const YGWrap_YGWrapNoWrap: YGWrap = 0;
pub const YGWrap_YGWrapWrap: YGWrap = 1;
pub const YGWrap_YGWrapWrapReverse: YGWrap = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct YGSize {
    #[offset(0)]
    pub width: (),
    #[offset(4)]
    pub height: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct YGValue {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    pub unit: YGUnit,
}
pub fn __compareDoubles_3(mut a: (), mut b: ()) -> () {
    let mut arg1: () = (a.reinterpret_cast::<()>().read());
    let mut arg2: () = (b.reinterpret_cast::<()>().read());
    if (((arg1 < arg2) as ()) != 0) {
        return -1_();
    }
    if (((arg1 > arg2) as ()) != 0) {
        return 1_();
    }
    return 0_();
}
pub fn __printBenchmarkResult_4(mut name: Ptr<()>, mut start: (), mut endTimes: Ptr<()>) {
    let timesInMs: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1000).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut mean: () = 0_();
    let mut lastEnd: () = start;
    let mut i: () = 0_();
    'loop_: while (((i < 1000_()) as ()) != 0) {
        (*timesInMs.borrow_mut())[(i) as usize] = {
            (((({ (elem!(endTimes, i).read()) } - { lastEnd }) as ()) / ((1000000_() as ()) as ()))
                * 1000_())
        };
        lastEnd = { (elem!(endTimes, i).read()) };
        mean += (*timesInMs.borrow())[(i) as usize];
        i.postfix_inc();
    }
    mean /= 1000_();
    unimplemented!();
    let mut median: () = (*timesInMs.borrow())[(1000_() / 2_()) as usize];
    let mut variance: () = 0_();
    let mut i: () = 0_();
    'loop_: while (((i < 1000_()) as ()) != 0) {
        variance += unimplemented!();
        i.postfix_inc();
    }
    variance /= 1000_();
    let mut stddev: () = unimplemented!();
    println!(
        "{}: median: {:.6} ms, stddev: {:.6} ms",
        name, median, stddev
    );
}
pub fn _measure_5(
    mut node: Ptr<YGNode>,
    mut width: (),
    mut widthMode: YGMeasureMode,
    mut height: (),
    mut heightMode: YGMeasureMode,
) -> YGSize {
    &(node);
    return YGSize {
        width: if ((((widthMode as ()) == (YGMeasureMode_YGMeasureModeUndefined as i32)) as ())
            != 0)
        {
            10_()
        } else {
            width
        },
        height: if ((((heightMode as ()) == (YGMeasureMode_YGMeasureModeUndefined as i32)) as ())
            != 0)
        {
            10_()
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
fn main_0(argc: i32, argv: Ptr<Ptr<i8>>) -> () {
    &(argc);
    &(argv);
    let mut __start: () = unimplemented!();
    let __endTimes: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1000).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    {
        {
            __start = ({ clock_6() });
            let mut __i: () = 0_();
            'loop_: while (((__i < 1000_()) as ()) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_7() });
                        ({ YGNodeStyleSetWidth_8((root).clone(), 100_()) });
                        ({ YGNodeStyleSetHeight_9((root).clone(), 100_()) });
                        let mut i: () = 0_();
                        'loop_: while (((i < 10_()) as ()) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_7() });
                            ({
                                YGNodeSetMeasureFunc_10(
                                    (child).clone(),
                                    FnPtr::<
                                        fn(
                                            Ptr<YGNode>,
                                            (),
                                            YGMeasureMode,
                                            (),
                                            YGMeasureMode,
                                        ) -> YGSize,
                                    >::new(_measure_5),
                                )
                            });
                            ({ YGNodeStyleSetFlex_11((child).clone(), 1_()) });
                            ({ YGNodeInsertChild_12((root).clone(), (child).clone(), 0_()) });
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_14((root).clone()) });
                    }
                }
                let __rhs = ({ clock_6() });
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<()>::from_string_literal(b"Stack with flex"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<()>),
                )
            });
            __start = ({ clock_6() });
            let mut __i: () = 0_();
            'loop_: while (((__i < 1000_()) as ()) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_7() });
                        let mut i: () = 0_();
                        'loop_: while (((i < 10_()) as ()) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_7() });
                            ({ YGNodeStyleSetHeight_9((child).clone(), 20_()) });
                            ({
                                YGNodeSetMeasureFunc_10(
                                    (child).clone(),
                                    FnPtr::<
                                        fn(
                                            Ptr<YGNode>,
                                            (),
                                            YGMeasureMode,
                                            (),
                                            YGMeasureMode,
                                        ) -> YGSize,
                                    >::new(_measure_5),
                                )
                            });
                            ({ YGNodeInsertChild_12((root).clone(), (child).clone(), 0_()) });
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_14((root).clone()) });
                    }
                }
                let __rhs = ({ clock_6() });
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<()>::from_string_literal(b"Align stretch in undefined axis"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<()>),
                )
            });
            __start = ({ clock_6() });
            let mut __i: () = 0_();
            'loop_: while (((__i < 1000_()) as ()) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_7() });
                        let mut i: () = 0_();
                        'loop_: while (((i < 10_()) as ()) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_7() });
                            ({ YGNodeStyleSetFlex_11((child).clone(), 1_()) });
                            ({ YGNodeInsertChild_12((root).clone(), (child).clone(), 0_()) });
                            let mut ii: () = 0_();
                            'loop_: while (((ii < 10_()) as ()) != 0) {
                                let mut grandChild: Ptr<YGNode> = ({ YGNodeNew_7() });
                                ({
                                    YGNodeSetMeasureFunc_10(
                                        (grandChild).clone(),
                                        FnPtr::<
                                            fn(
                                                Ptr<YGNode>,
                                                (),
                                                YGMeasureMode,
                                                (),
                                                YGMeasureMode,
                                            )
                                                -> YGSize,
                                        >::new(_measure_5),
                                    )
                                });
                                ({ YGNodeStyleSetFlex_11((grandChild).clone(), 1_()) });
                                ({
                                    YGNodeInsertChild_12(
                                        (child).clone(),
                                        (grandChild).clone(),
                                        0_(),
                                    )
                                });
                                ii.postfix_inc();
                            }
                            i.postfix_inc();
                        }
                        ({
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_14((root).clone()) });
                    }
                }
                let __rhs = ({ clock_6() });
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<()>::from_string_literal(b"Nested flex"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<()>),
                )
            });
            __start = ({ clock_6() });
            let mut __i: () = 0_();
            'loop_: while (((__i < 1000_()) as ()) != 0) {
                {
                    {
                        let mut root: Ptr<YGNode> = ({ YGNodeNew_7() });
                        let mut i: () = 0_();
                        'loop_: while (((i < 10_()) as ()) != 0) {
                            let mut child: Ptr<YGNode> = ({ YGNodeNew_7() });
                            ({ YGNodeStyleSetFlexGrow_15((child).clone(), 1_()) });
                            ({ YGNodeStyleSetWidth_8((child).clone(), 10_()) });
                            ({ YGNodeStyleSetHeight_9((child).clone(), 10_()) });
                            ({ YGNodeInsertChild_12((root).clone(), (child).clone(), 0_()) });
                            let mut ii: () = 0_();
                            'loop_: while (((ii < 10_()) as ()) != 0) {
                                let mut grandChild: Ptr<YGNode> = ({ YGNodeNew_7() });
                                ({
                                    YGNodeStyleSetFlexDirection_16(
                                        (grandChild).clone(),
                                        YGFlexDirection_YGFlexDirectionRow,
                                    )
                                });
                                ({ YGNodeStyleSetFlexGrow_15((grandChild).clone(), 1_()) });
                                ({ YGNodeStyleSetWidth_8((grandChild).clone(), 10_()) });
                                ({ YGNodeStyleSetHeight_9((grandChild).clone(), 10_()) });
                                ({
                                    YGNodeInsertChild_12(
                                        (child).clone(),
                                        (grandChild).clone(),
                                        0_(),
                                    )
                                });
                                let mut iii: () = 0_();
                                'loop_: while (((iii < 10_()) as ()) != 0) {
                                    let mut grandGrandChild: Ptr<YGNode> = ({ YGNodeNew_7() });
                                    ({
                                        YGNodeStyleSetFlexGrow_15((grandGrandChild).clone(), 1_())
                                    });
                                    ({ YGNodeStyleSetWidth_8((grandGrandChild).clone(), 10_()) });
                                    ({ YGNodeStyleSetHeight_9((grandGrandChild).clone(), 10_()) });
                                    ({
                                        YGNodeInsertChild_12(
                                            (grandChild).clone(),
                                            (grandGrandChild).clone(),
                                            0_(),
                                        )
                                    });
                                    let mut iiii: () = 0_();
                                    'loop_: while (((iiii < 10_()) as ()) != 0) {
                                        let mut grandGrandGrandChild: Ptr<YGNode> =
                                            ({ YGNodeNew_7() });
                                        ({
                                            YGNodeStyleSetFlexDirection_16(
                                                (grandGrandGrandChild).clone(),
                                                YGFlexDirection_YGFlexDirectionRow,
                                            )
                                        });
                                        ({
                                            YGNodeStyleSetFlexGrow_15(
                                                (grandGrandGrandChild).clone(),
                                                1_(),
                                            )
                                        });
                                        ({
                                            YGNodeStyleSetWidth_8(
                                                (grandGrandGrandChild).clone(),
                                                10_(),
                                            )
                                        });
                                        ({
                                            YGNodeStyleSetHeight_9(
                                                (grandGrandGrandChild).clone(),
                                                10_(),
                                            )
                                        });
                                        ({
                                            YGNodeInsertChild_12(
                                                (grandGrandChild).clone(),
                                                (grandGrandGrandChild).clone(),
                                                0_(),
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
                            let _availableWidth: () = (unimplemented!());
                            let _availableHeight: () = (unimplemented!());
                            YGNodeCalculateLayout_13(
                                (root).clone(),
                                _availableWidth,
                                _availableHeight,
                                YGDirection_YGDirectionLTR,
                            )
                        });
                        ({ YGNodeFreeRecursive_14((root).clone()) });
                    }
                }
                let __rhs = ({ clock_6() });
                (*__endTimes.borrow_mut())[(__i) as usize] = __rhs;
                __i.postfix_inc();
            }
            ({
                __printBenchmarkResult_4(
                    Ptr::<()>::from_string_literal(b"Huge nested layout"),
                    __start,
                    (__endTimes.as_pointer() as Ptr<()>),
                )
            });
        }
    }
    return 0_();
}
pub fn operator_bitnot_17(mut a: YGErrata) -> YGErrata {
    return ((!(a as ())) as YGErrata);
}
pub fn operator_bitor_18(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as ()) | (b as ())) as YGErrata);
}
pub fn operator_bitand_19(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as ()) & (b as ())) as YGErrata);
}
pub fn operator_bitxor_20(mut a: YGErrata, mut b: YGErrata) -> YGErrata {
    return (((a as ()) ^ (b as ())) as YGErrata);
}
pub fn operator_bitor_assign_21(a: Ptr<YGErrata>, mut b: YGErrata) -> Ptr<YGErrata> {
    return {
        {
            let __rhs = (b as ());
            a.reinterpret_cast::<()>()
                .with_mut(|__v| *__v = *__v | __rhs)
        };
        a.reinterpret_cast::<()>()
    }
    .reinterpret_cast::<YGErrata>();
}
pub fn operator_bitand_assign_22(a: Ptr<YGErrata>, mut b: YGErrata) -> Ptr<YGErrata> {
    return {
        {
            let __rhs = (b as ());
            a.reinterpret_cast::<()>()
                .with_mut(|__v| *__v = *__v & __rhs)
        };
        a.reinterpret_cast::<()>()
    }
    .reinterpret_cast::<YGErrata>();
}
pub fn operator_bitxor_assign_23(a: Ptr<YGErrata>, mut b: YGErrata) -> Ptr<YGErrata> {
    return {
        {
            let __rhs = (b as ());
            a.reinterpret_cast::<()>()
                .with_mut(|__v| *__v = *__v ^ __rhs)
        };
        a.reinterpret_cast::<()>()
    }
    .reinterpret_cast::<YGErrata>();
}
thread_local!(
    pub static YGUndefined_24: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn operator_eq_25(lhs: Ptr<YGValue>, rhs: Ptr<YGValue>) -> () {
    if ({ (lhs.with(|__s| __s.unit) as ()) } != { (rhs.with(|__s| __s.unit) as ()) }) {
        return false;
    }
    'switch: {
        match { (lhs.with(|__s| __s.unit) as ()) } {
            __v if __v == (YGUnit_YGUnitUndefined as ())
                || __v == (YGUnit_YGUnitAuto as ())
                || __v == (YGUnit_YGUnitFitContent as ())
                || __v == (YGUnit_YGUnitMaxContent as ())
                || __v == (YGUnit_YGUnitStretch as ()) =>
            {
                return true;
            }
            __v if __v == (YGUnit_YGUnitPoint as ()) || __v == (YGUnit_YGUnitPercent as ()) => {
                return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
            }
            _ => {
                return false;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn operator_neg_26(value: Ptr<YGValue>) -> YGValue {
    return YGValue {
        value: -value.with(|__s| __s.value),
        unit: value.with(|__s| __s.unit),
    };
}
pub fn bitCount_27() -> () {
    return unimplemented!();
}
pub fn bitCount_28() -> () {
    return unimplemented!();
}
pub fn bitCount_29() -> () {
    return unimplemented!();
}
pub fn bitCount_30() -> () {
    return unimplemented!();
}
pub fn bitCount_31() -> () {
    return unimplemented!();
}
pub fn bitCount_32() -> () {
    return unimplemented!();
}
pub fn bitCount_33() -> () {
    return unimplemented!();
}
pub fn bitCount_34() -> () {
    return unimplemented!();
}
pub fn bitCount_35() -> () {
    return unimplemented!();
}
pub fn bitCount_36() -> () {
    return unimplemented!();
}
pub fn to_underlying_37(e: facebook_yoga_Edge) -> () {
    let e: Value<facebook_yoga_Edge> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as ());
}
pub fn to_underlying_38(e: facebook_yoga_Dimension) -> () {
    let e: Value<facebook_yoga_Dimension> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as ());
}
pub fn to_underlying_39(e: facebook_yoga_PhysicalEdge) -> () {
    let e: Value<facebook_yoga_PhysicalEdge> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as ());
}
pub fn to_underlying_40(e: facebook_yoga_Gutter) -> () {
    let e: Value<facebook_yoga_Gutter> = Rc::new(RefCell::new(e));
    return ((*e.borrow()) as ());
}
pub type facebook_yoga_Errata = ();
pub const facebook_yoga_Errata_None: facebook_yoga_Errata = 0;
pub const facebook_yoga_Errata_StretchFlexBasis: facebook_yoga_Errata = 1;
pub const facebook_yoga_Errata_AbsolutePositionWithoutInsetsExcludesPadding: facebook_yoga_Errata =
    2;
pub const facebook_yoga_Errata_AbsolutePercentAgainstInnerSize: facebook_yoga_Errata = 4;
pub const facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto: facebook_yoga_Errata = 8;
pub const facebook_yoga_Errata_FlexFirstPassUsesRunningTotals: facebook_yoga_Errata = 16;
pub const facebook_yoga_Errata_All: facebook_yoga_Errata = 2147483647;
pub const facebook_yoga_Errata_Classic: facebook_yoga_Errata = 2147483646;
pub fn operator_bitnot_41(mut a: facebook_yoga_Errata) -> facebook_yoga_Errata {
    return ((!(a as ())) as facebook_yoga_Errata);
}
pub fn operator_bitor_42(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as ()) | (b as ())) as facebook_yoga_Errata);
}
pub fn operator_bitand_43(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as ()) & (b as ())) as facebook_yoga_Errata);
}
pub fn operator_bitxor_44(
    mut a: facebook_yoga_Errata,
    mut b: facebook_yoga_Errata,
) -> facebook_yoga_Errata {
    return (((a as ()) ^ (b as ())) as facebook_yoga_Errata);
}
pub fn operator_bitor_assign_45(
    a: Ptr<facebook_yoga_Errata>,
    mut b: facebook_yoga_Errata,
) -> Ptr<facebook_yoga_Errata> {
    return {
        {
            let __rhs = (b as ());
            a.reinterpret_cast::<()>()
                .with_mut(|__v| *__v = *__v | __rhs)
        };
        a.reinterpret_cast::<()>()
    }
    .reinterpret_cast::<facebook_yoga_Errata>();
}
pub fn operator_bitand_assign_46(
    a: Ptr<facebook_yoga_Errata>,
    mut b: facebook_yoga_Errata,
) -> Ptr<facebook_yoga_Errata> {
    return {
        {
            let __rhs = (b as ());
            a.reinterpret_cast::<()>()
                .with_mut(|__v| *__v = *__v & __rhs)
        };
        a.reinterpret_cast::<()>()
    }
    .reinterpret_cast::<facebook_yoga_Errata>();
}
pub fn operator_bitxor_assign_47(
    a: Ptr<facebook_yoga_Errata>,
    mut b: facebook_yoga_Errata,
) -> Ptr<facebook_yoga_Errata> {
    return {
        {
            let __rhs = (b as ());
            a.reinterpret_cast::<()>()
                .with_mut(|__v| *__v = *__v ^ __rhs)
        };
        a.reinterpret_cast::<()>()
    }
    .reinterpret_cast::<facebook_yoga_Errata>();
}
pub fn scopedEnum_48(mut unscoped: YGErrata) -> facebook_yoga_Errata {
    return (unscoped as facebook_yoga_Errata);
}
pub fn unscopedEnum_49(mut scoped: facebook_yoga_Errata) -> YGErrata {
    return (scoped as YGErrata);
}
pub fn toString_50(mut e: facebook_yoga_Errata) -> Ptr<()> {
    return ({ YGErrataToString_51(({ unscopedEnum_49(e) })) });
}
pub type facebook_yoga_ExperimentalFeature = ();
pub const facebook_yoga_ExperimentalFeature_WebFlexBasis: facebook_yoga_ExperimentalFeature = 0;
pub const facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent:
    facebook_yoga_ExperimentalFeature = 1;
pub fn ordinalCount_52() -> () {
    return 2_();
}
pub fn scopedEnum_53(mut unscoped: YGExperimentalFeature) -> facebook_yoga_ExperimentalFeature {
    return (unscoped as facebook_yoga_ExperimentalFeature);
}
pub fn unscopedEnum_54(mut scoped: facebook_yoga_ExperimentalFeature) -> YGExperimentalFeature {
    return (scoped as YGExperimentalFeature);
}
pub fn toString_55(mut e: facebook_yoga_ExperimentalFeature) -> Ptr<()> {
    return ({ YGExperimentalFeatureToString_56(({ unscopedEnum_54(e) })) });
}
pub type facebook_yoga_LogLevel = ();
pub const facebook_yoga_LogLevel_Error: facebook_yoga_LogLevel = 0;
pub const facebook_yoga_LogLevel_Warn: facebook_yoga_LogLevel = 1;
pub const facebook_yoga_LogLevel_Info: facebook_yoga_LogLevel = 2;
pub const facebook_yoga_LogLevel_Debug: facebook_yoga_LogLevel = 3;
pub const facebook_yoga_LogLevel_Verbose: facebook_yoga_LogLevel = 4;
pub const facebook_yoga_LogLevel_Fatal: facebook_yoga_LogLevel = 5;
pub fn ordinalCount_57() -> () {
    return 6_();
}
pub fn scopedEnum_58(mut unscoped: YGLogLevel) -> facebook_yoga_LogLevel {
    return (unscoped as facebook_yoga_LogLevel);
}
pub fn unscopedEnum_59(mut scoped: facebook_yoga_LogLevel) -> YGLogLevel {
    return (scoped as YGLogLevel);
}
pub fn toString_60(mut e: facebook_yoga_LogLevel) -> Ptr<()> {
    return ({ YGLogLevelToString_61(({ unscopedEnum_59(e) })) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct YGConfig {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct facebook_yoga_Config {
    #[offset(0)]
    #[byte_size(8)]
    cloneNodeCallback_: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, ()) -> Ptr<YGNode>>,
    #[offset(8)]
    #[byte_size(8)]
    logger_: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>,
    #[offset(16)]
    useWebDefaults_: (),
    #[offset(20)]
    version_: (),
    #[offset(24)]
    #[byte_size(8)]
    experimentalFeatures_: (),
    #[offset(32)]
    #[byte_size(4)]
    errata_: facebook_yoga_Errata,
    #[offset(36)]
    pointScaleFactor_: (),
    #[offset(40)]
    #[byte_size(8)]
    context_: (),
}
impl facebook_yoga_Config {
    pub fn new(
        mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>,
    ) -> Self {
        Self {
            cloneNodeCallback_: FnPtr::<fn(Ptr<YGNode>, Ptr<YGNode>, ()) -> Ptr<YGNode>>::null(),
            logger_: (logger).clone(),
            useWebDefaults_: false,
            version_: 0_(),
            experimentalFeatures_: unimplemented!(),
            errata_: ({
                operator_bitor_42(
                    facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                    facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                )
            }),
            pointScaleFactor_: 1_f32,
            context_: unimplemented!(),
        }
    }
    pub fn getDefault() -> Ptr<facebook_yoga_Config> {
        return defaultConfig__62.with(|v| v.as_pointer());
    }
}
impl Default for facebook_yoga_Config {
    fn default() -> Self {
        {
            facebook_yoga_Config {
                cloneNodeCallback_: FnPtr::<fn(Ptr<YGNode>, Ptr<YGNode>, ()) -> Ptr<YGNode>>::null(
                ),
                logger_: FnPtr::<
                    fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> (),
                >::null(),
                useWebDefaults_: false,
                version_: 0_(),
                experimentalFeatures_: unimplemented!(),
                errata_: ({
                    operator_bitor_42(
                        facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto,
                        facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
                    )
                }),
                pointScaleFactor_: 1_f32,
                context_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static defaultConfig__62: Value<facebook_yoga_Config> =
        Rc::new(RefCell::new(facebook_yoga_Config::new({
            FnPtr::<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>::null()
        })));
);
pub fn resolveRef_63(mut ref_: Ptr<YGConfig>) -> Ptr<facebook_yoga_Config> {
    return ref_.reinterpret_cast::<facebook_yoga_Config>();
}
pub fn resolveRef_64(mut ref_: Ptr<YGConfig>) -> Ptr<facebook_yoga_Config> {
    return ref_.reinterpret_cast::<facebook_yoga_Config>();
}
pub type facebook_yoga_Display = ();
pub const facebook_yoga_Display_Flex: facebook_yoga_Display = 0;
pub const facebook_yoga_Display_None: facebook_yoga_Display = 1;
pub const facebook_yoga_Display_Contents: facebook_yoga_Display = 2;
pub const facebook_yoga_Display_Grid: facebook_yoga_Display = 3;
pub fn ordinalCount_65() -> () {
    return 4_();
}
pub fn scopedEnum_66(mut unscoped: YGDisplay) -> facebook_yoga_Display {
    return (unscoped as facebook_yoga_Display);
}
pub fn unscopedEnum_67(mut scoped: facebook_yoga_Display) -> YGDisplay {
    return (scoped as YGDisplay);
}
pub fn toString_68(mut e: facebook_yoga_Display) -> Ptr<()> {
    return ({ YGDisplayToString_69(({ unscopedEnum_67(e) })) });
}
pub fn operator_ne_70(
    a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    b: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
) -> () {
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
    childIndex_: (),
    #[offset(16)]
    #[byte_size(8)]
    backtrack_: (),
}
impl facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    pub fn new_1(mut node: Ptr<facebook_yoga_Node>, mut childIndex: ()) -> Self {
        Self {
            node_: (node).clone(),
            childIndex_: childIndex,
            backtrack_: unimplemented!(),
        }
    }
    pub fn move_from(
        _a0: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    ) -> Self {
        Self {
            node_: { (*_a0.upgrade().deref()).node_.clone() },
            childIndex_: { (*_a0.upgrade().deref()).childIndex_ },
            backtrack_: unimplemented!(),
        }
    }
}
impl Default for facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
    fn default() -> Self {
        {
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator {
                node_: Ptr::<facebook_yoga_Node>::null(),
                childIndex_: 0_(),
                backtrack_: unimplemented!(),
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
pub type facebook_yoga_Dimension = ();
pub const facebook_yoga_Dimension_Width: facebook_yoga_Dimension = 0;
pub const facebook_yoga_Dimension_Height: facebook_yoga_Dimension = 1;
pub fn ordinalCount_71() -> () {
    return 2_();
}
pub fn scopedEnum_72(mut unscoped: YGDimension) -> facebook_yoga_Dimension {
    return (unscoped as facebook_yoga_Dimension);
}
pub fn unscopedEnum_73(mut scoped: facebook_yoga_Dimension) -> YGDimension {
    return (scoped as YGDimension);
}
pub fn toString_74(mut e: facebook_yoga_Dimension) -> Ptr<()> {
    return ({ YGDimensionToString_75(({ unscopedEnum_73(e) })) });
}
pub type facebook_yoga_Direction = ();
pub const facebook_yoga_Direction_Inherit: facebook_yoga_Direction = 0;
pub const facebook_yoga_Direction_LTR: facebook_yoga_Direction = 1;
pub const facebook_yoga_Direction_RTL: facebook_yoga_Direction = 2;
pub fn ordinalCount_76() -> () {
    return 3_();
}
pub fn scopedEnum_77(mut unscoped: YGDirection) -> facebook_yoga_Direction {
    return (unscoped as facebook_yoga_Direction);
}
pub fn unscopedEnum_78(mut scoped: facebook_yoga_Direction) -> YGDirection {
    return (scoped as YGDirection);
}
pub fn toString_79(mut e: facebook_yoga_Direction) -> Ptr<()> {
    return ({ YGDirectionToString_80(({ unscopedEnum_78(e) })) });
}
pub type facebook_yoga_Edge = ();
pub const facebook_yoga_Edge_Left: facebook_yoga_Edge = 0;
pub const facebook_yoga_Edge_Top: facebook_yoga_Edge = 1;
pub const facebook_yoga_Edge_Right: facebook_yoga_Edge = 2;
pub const facebook_yoga_Edge_Bottom: facebook_yoga_Edge = 3;
pub const facebook_yoga_Edge_Start: facebook_yoga_Edge = 4;
pub const facebook_yoga_Edge_End: facebook_yoga_Edge = 5;
pub const facebook_yoga_Edge_Horizontal: facebook_yoga_Edge = 6;
pub const facebook_yoga_Edge_Vertical: facebook_yoga_Edge = 7;
pub const facebook_yoga_Edge_All: facebook_yoga_Edge = 8;
pub fn ordinalCount_81() -> () {
    return 9_();
}
pub fn scopedEnum_82(mut unscoped: YGEdge) -> facebook_yoga_Edge {
    return (unscoped as facebook_yoga_Edge);
}
pub fn unscopedEnum_83(mut scoped: facebook_yoga_Edge) -> YGEdge {
    return (scoped as YGEdge);
}
pub fn toString_84(mut e: facebook_yoga_Edge) -> Ptr<()> {
    return ({ YGEdgeToString_85(({ unscopedEnum_83(e) })) });
}
pub type facebook_yoga_MeasureMode = ();
pub const facebook_yoga_MeasureMode_Undefined: facebook_yoga_MeasureMode = 0;
pub const facebook_yoga_MeasureMode_Exactly: facebook_yoga_MeasureMode = 1;
pub const facebook_yoga_MeasureMode_AtMost: facebook_yoga_MeasureMode = 2;
pub fn ordinalCount_86() -> () {
    return 3_();
}
pub fn scopedEnum_87(mut unscoped: YGMeasureMode) -> facebook_yoga_MeasureMode {
    return (unscoped as facebook_yoga_MeasureMode);
}
pub fn unscopedEnum_88(mut scoped: facebook_yoga_MeasureMode) -> YGMeasureMode {
    return (scoped as YGMeasureMode);
}
pub fn toString_89(mut e: facebook_yoga_MeasureMode) -> Ptr<()> {
    return ({ YGMeasureModeToString_90(({ unscopedEnum_88(e) })) });
}
pub type facebook_yoga_NodeType = ();
pub const facebook_yoga_NodeType_Default: facebook_yoga_NodeType = 0;
pub const facebook_yoga_NodeType_Text: facebook_yoga_NodeType = 1;
pub fn ordinalCount_91() -> () {
    return 2_();
}
pub fn scopedEnum_92(mut unscoped: YGNodeType) -> facebook_yoga_NodeType {
    return (unscoped as facebook_yoga_NodeType);
}
pub fn unscopedEnum_93(mut scoped: facebook_yoga_NodeType) -> YGNodeType {
    return (scoped as YGNodeType);
}
pub fn toString_94(mut e: facebook_yoga_NodeType) -> Ptr<()> {
    return ({ YGNodeTypeToString_95(({ unscopedEnum_93(e) })) });
}
pub type facebook_yoga_PhysicalEdge = ();
pub const facebook_yoga_PhysicalEdge_Left: facebook_yoga_PhysicalEdge = 0;
pub const facebook_yoga_PhysicalEdge_Top: facebook_yoga_PhysicalEdge = 1;
pub const facebook_yoga_PhysicalEdge_Right: facebook_yoga_PhysicalEdge = 2;
pub const facebook_yoga_PhysicalEdge_Bottom: facebook_yoga_PhysicalEdge = 3;
pub type facebook_yoga_SizingMode = ();
pub const facebook_yoga_SizingMode_StretchFit: facebook_yoga_SizingMode = 0;
pub const facebook_yoga_SizingMode_MaxContent: facebook_yoga_SizingMode = 1;
pub const facebook_yoga_SizingMode_FitContent: facebook_yoga_SizingMode = 2;
pub fn measureMode_96(mut mode: facebook_yoga_SizingMode) -> facebook_yoga_MeasureMode {
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
            ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid SizingMode")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn sizingMode_98(mut mode: facebook_yoga_MeasureMode) -> facebook_yoga_SizingMode {
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
            ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid MeasureMode")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn isUndefined_99(mut value: ()) -> () {
    return (value != value);
}
pub fn isUndefined_100(mut value: ()) -> () {
    return (value != value);
}
pub fn isDefined_101(mut value: ()) -> () {
    return !({ isUndefined_99(value) });
}
pub fn isDefined_102(mut value: ()) -> () {
    return !({ isUndefined_100(value) });
}
pub fn isinf_103(value: ()) -> () {
    let value: Value<()> = Rc::new(RefCell::new(value));
    return ((*value.borrow()) == unimplemented!()) || ((*value.borrow()) == -unimplemented!());
}
pub fn maxOrDefined_104(mut a: (), mut b: ()) -> () {
    if ({ isDefined_101(a) }) && ({ isDefined_101(b) }) {
        return (unimplemented!().read());
    }
    return if ({ isUndefined_99(a) }) { b } else { a };
}
pub fn inexactEquals_105(mut a: (), mut b: ()) -> () {
    if ({ isDefined_101(a) }) && ({ isDefined_101(b) }) {
        return (unimplemented!() < 1e-04_f32);
    }
    return ({ isUndefined_99(a) }) && ({ isUndefined_99(b) });
}
pub fn inexactEquals_106(mut a: (), mut b: ()) -> () {
    if ({ isDefined_102(a) }) && ({ isDefined_102(b) }) {
        return (unimplemented!() < 1e-04_f64);
    }
    return ({ isUndefined_100(a) }) && ({ isUndefined_100(b) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct facebook_yoga_CachedMeasurement {
    #[offset(0)]
    pub availableWidth: (),
    #[offset(4)]
    pub availableHeight: (),
    #[offset(8)]
    #[byte_size(4)]
    pub widthSizingMode: facebook_yoga_SizingMode,
    #[offset(12)]
    #[byte_size(4)]
    pub heightSizingMode: facebook_yoga_SizingMode,
    #[offset(16)]
    pub computedWidth: (),
    #[offset(20)]
    pub computedHeight: (),
    #[offset(24)]
    pub ownerWidth: (),
    #[offset(28)]
    pub ownerHeight: (),
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
                availableWidth: -1_(),
                availableHeight: -1_(),
                widthSizingMode: facebook_yoga_SizingMode_MaxContent,
                heightSizingMode: facebook_yoga_SizingMode_MaxContent,
                computedWidth: -1_(),
                computedHeight: -1_(),
                ownerWidth: -1_(),
                ownerHeight: -1_(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct facebook_yoga_FloatOptional {
    #[offset(0)]
    value_: (),
}
impl facebook_yoga_FloatOptional {
    pub fn new_1(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl CppLess for facebook_yoga_FloatOptional {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_107(__lhs.read(), __rhs.read())
    }
}
impl CppEq for facebook_yoga_FloatOptional {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_108(__lhs.read(), __rhs.read())
    }
}
impl Default for facebook_yoga_FloatOptional {
    fn default() -> Self {
        {
            facebook_yoga_FloatOptional {
                value_: unimplemented!(),
            }
        }
    }
}
pub fn operator_eq_108(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> () {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
        == ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }))
        || (({ facebook_yoga_FloatOptionalImpl::isUndefined(&lhs.as_pointer()) })
            && ({ facebook_yoga_FloatOptionalImpl::isUndefined(&rhs.as_pointer()) }));
}
pub fn operator_eq_109(mut lhs: facebook_yoga_FloatOptional, mut rhs: ()) -> () {
    return ({
        let _lhs: facebook_yoga_FloatOptional = lhs;
        operator_eq_108(_lhs, facebook_yoga_FloatOptional::new_1({ rhs }))
    });
}
pub fn operator_eq_110(mut lhs: (), mut rhs: facebook_yoga_FloatOptional) -> () {
    return ({
        let _lhs: facebook_yoga_FloatOptional = rhs;
        operator_eq_109(_lhs, lhs)
    });
}
pub fn operator_add_111(
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
pub fn operator_gt_112(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> () {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
        > ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }));
}
pub fn operator_lt_107(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> () {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return (({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) })
        < ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) }));
}
pub fn operator_ge_113(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_gt_112(_lhs, (rhs).clone())
    }) || ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_eq_108(_lhs, (rhs).clone())
    });
}
pub fn operator_le_114(
    mut lhs: facebook_yoga_FloatOptional,
    mut rhs: facebook_yoga_FloatOptional,
) -> () {
    return ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_lt_107(_lhs, (rhs).clone())
    }) || ({
        let _lhs: facebook_yoga_FloatOptional = (lhs).clone();
        operator_eq_108(_lhs, (rhs).clone())
    });
}
pub fn maxOrDefined_115(
    lhs: facebook_yoga_FloatOptional,
    rhs: facebook_yoga_FloatOptional,
) -> facebook_yoga_FloatOptional {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return facebook_yoga_FloatOptional::new_1({
        ({
            let _a: () = ({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) });
            let _b: () = ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) });
            maxOrDefined_104(_a, _b)
        })
    });
}
pub fn inexactEquals_116(lhs: facebook_yoga_FloatOptional, rhs: facebook_yoga_FloatOptional) -> () {
    let lhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(lhs));
    let rhs: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(rhs));
    return ({
        let _a: () = ({ facebook_yoga_FloatOptionalImpl::unwrap(&lhs.as_pointer()) });
        let _b: () = ({ facebook_yoga_FloatOptionalImpl::unwrap(&rhs.as_pointer()) });
        inexactEquals_105(_a, _b)
    });
}
thread_local!(
    pub static MaxCachedMeasurements_117: Value<()> = Rc::new(RefCell::new(8_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(432)]
pub struct facebook_yoga_LayoutResults {
    #[offset(0)]
    pub computedFlexBasisGeneration: (),
    #[offset(4)]
    #[byte_size(4)]
    pub computedFlexBasis: facebook_yoga_FloatOptional,
    #[offset(8)]
    #[byte_size(4)]
    pub computedAutoMinMainSize: facebook_yoga_FloatOptional,
    #[offset(12)]
    #[byte_size(8)]
    pub minContentGeneration: (),
    #[offset(20)]
    #[byte_size(8)]
    pub minContentOwnerWidth: (),
    #[offset(28)]
    #[byte_size(8)]
    pub minContent: (),
    #[offset(36)]
    pub generationCount: (),
    #[offset(40)]
    pub configVersion: (),
    #[offset(44)]
    #[byte_size(1)]
    pub lastOwnerDirection: facebook_yoga_Direction,
    #[offset(48)]
    pub nextCachedMeasurementsIndex: (),
    #[offset(52)]
    #[byte_size(256)]
    pub cachedMeasurements: (),
    #[offset(308)]
    #[byte_size(32)]
    pub cachedLayout: facebook_yoga_CachedMeasurement,
    #[offset(340)]
    #[byte_size(1)]
    direction_: facebook_yoga_Direction,
    #[offset(340)]
    hadOverflow_: (),
    #[offset(344)]
    #[byte_size(8)]
    dimensions_: (),
    #[offset(352)]
    #[byte_size(8)]
    measuredDimensions_: (),
    #[offset(360)]
    #[byte_size(8)]
    rawDimensions_: (),
    #[offset(368)]
    #[byte_size(16)]
    position_: (),
    #[offset(384)]
    #[byte_size(16)]
    margin_: (),
    #[offset(400)]
    #[byte_size(16)]
    border_: (),
    #[offset(416)]
    #[byte_size(16)]
    padding_: (),
}
impl Default for facebook_yoga_LayoutResults {
    fn default() -> Self {
        {
            facebook_yoga_LayoutResults {
                computedFlexBasisGeneration: 0_(),
                computedFlexBasis: <facebook_yoga_FloatOptional>::default(),
                computedAutoMinMainSize: <facebook_yoga_FloatOptional>::default(),
                minContentGeneration: unimplemented!(),
                minContentOwnerWidth: unimplemented!(),
                minContent: unimplemented!(),
                generationCount: 0_(),
                configVersion: 0_(),
                lastOwnerDirection: facebook_yoga_Direction_Inherit,
                nextCachedMeasurementsIndex: 0_(),
                cachedMeasurements: unimplemented!(),
                cachedLayout: <facebook_yoga_CachedMeasurement>::default(),
                direction_: facebook_yoga_Direction_Inherit,
                hadOverflow_: false,
                dimensions_: unimplemented!(),
                measuredDimensions_: unimplemented!(),
                rawDimensions_: unimplemented!(),
                position_: unimplemented!(),
                margin_: unimplemented!(),
                border_: unimplemented!(),
                padding_: unimplemented!(),
            }
        }
    }
}
pub type facebook_yoga_FlexDirection = ();
pub const facebook_yoga_FlexDirection_Column: facebook_yoga_FlexDirection = 0;
pub const facebook_yoga_FlexDirection_ColumnReverse: facebook_yoga_FlexDirection = 1;
pub const facebook_yoga_FlexDirection_Row: facebook_yoga_FlexDirection = 2;
pub const facebook_yoga_FlexDirection_RowReverse: facebook_yoga_FlexDirection = 3;
pub fn ordinalCount_118() -> () {
    return 4_();
}
pub fn scopedEnum_119(mut unscoped: YGFlexDirection) -> facebook_yoga_FlexDirection {
    return (unscoped as facebook_yoga_FlexDirection);
}
pub fn unscopedEnum_120(mut scoped: facebook_yoga_FlexDirection) -> YGFlexDirection {
    return (scoped as YGFlexDirection);
}
pub fn toString_121(mut e: facebook_yoga_FlexDirection) -> Ptr<()> {
    return ({ YGFlexDirectionToString_122(({ unscopedEnum_120(e) })) });
}
pub fn isRow_123(mut flexDirection: facebook_yoga_FlexDirection) -> () {
    return (flexDirection == facebook_yoga_FlexDirection_Row)
        || (flexDirection == facebook_yoga_FlexDirection_RowReverse);
}
pub fn isColumn_124(mut flexDirection: facebook_yoga_FlexDirection) -> () {
    return (flexDirection == facebook_yoga_FlexDirection_Column)
        || (flexDirection == facebook_yoga_FlexDirection_ColumnReverse);
}
pub fn resolveDirection_125(
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
pub fn resolveCrossDirection_126(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_FlexDirection {
    return if ({ isColumn_124(flexDirection) }) {
        ({ resolveDirection_125(facebook_yoga_FlexDirection_Row, direction) })
    } else {
        facebook_yoga_FlexDirection_Column
    };
}
pub fn flexStartEdge_127(
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
            ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid FlexDirection")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn flexEndEdge_128(
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
            ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid FlexDirection")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn inlineStartEdge_129(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if ({ isRow_123(flexDirection) }) {
        return if (direction == facebook_yoga_Direction_RTL) {
            facebook_yoga_PhysicalEdge_Right
        } else {
            facebook_yoga_PhysicalEdge_Left
        };
    }
    return facebook_yoga_PhysicalEdge_Top;
}
pub fn inlineEndEdge_130(
    mut flexDirection: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
) -> facebook_yoga_PhysicalEdge {
    if ({ isRow_123(flexDirection) }) {
        return if (direction == facebook_yoga_Direction_RTL) {
            facebook_yoga_PhysicalEdge_Left
        } else {
            facebook_yoga_PhysicalEdge_Right
        };
    }
    return facebook_yoga_PhysicalEdge_Bottom;
}
pub fn dimension_131(mut flexDirection: facebook_yoga_FlexDirection) -> facebook_yoga_Dimension {
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
            ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid FlexDirection")) });
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub type facebook_yoga_Align = ();
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
pub fn ordinalCount_132() -> () {
    return 11_();
}
pub fn scopedEnum_133(mut unscoped: YGAlign) -> facebook_yoga_Align {
    return (unscoped as facebook_yoga_Align);
}
pub fn unscopedEnum_134(mut scoped: facebook_yoga_Align) -> YGAlign {
    return (scoped as YGAlign);
}
pub fn toString_135(mut e: facebook_yoga_Align) -> Ptr<()> {
    return ({ YGAlignToString_136(({ unscopedEnum_134(e) })) });
}
pub type facebook_yoga_BoxSizing = ();
pub const facebook_yoga_BoxSizing_BorderBox: facebook_yoga_BoxSizing = 0;
pub const facebook_yoga_BoxSizing_ContentBox: facebook_yoga_BoxSizing = 1;
pub fn ordinalCount_137() -> () {
    return 2_();
}
pub fn scopedEnum_138(mut unscoped: YGBoxSizing) -> facebook_yoga_BoxSizing {
    return (unscoped as facebook_yoga_BoxSizing);
}
pub fn unscopedEnum_139(mut scoped: facebook_yoga_BoxSizing) -> YGBoxSizing {
    return (scoped as YGBoxSizing);
}
pub fn toString_140(mut e: facebook_yoga_BoxSizing) -> Ptr<()> {
    return ({ YGBoxSizingToString_141(({ unscopedEnum_139(e) })) });
}
pub type facebook_yoga_Gutter = ();
pub const facebook_yoga_Gutter_Column: facebook_yoga_Gutter = 0;
pub const facebook_yoga_Gutter_Row: facebook_yoga_Gutter = 1;
pub const facebook_yoga_Gutter_All: facebook_yoga_Gutter = 2;
pub fn ordinalCount_142() -> () {
    return 3_();
}
pub fn scopedEnum_143(mut unscoped: YGGutter) -> facebook_yoga_Gutter {
    return (unscoped as facebook_yoga_Gutter);
}
pub fn unscopedEnum_144(mut scoped: facebook_yoga_Gutter) -> YGGutter {
    return (scoped as YGGutter);
}
pub fn toString_145(mut e: facebook_yoga_Gutter) -> Ptr<()> {
    return ({ YGGutterToString_146(({ unscopedEnum_144(e) })) });
}
pub type facebook_yoga_Justify = ();
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
pub fn ordinalCount_147() -> () {
    return 10_();
}
pub fn scopedEnum_148(mut unscoped: YGJustify) -> facebook_yoga_Justify {
    return (unscoped as facebook_yoga_Justify);
}
pub fn unscopedEnum_149(mut scoped: facebook_yoga_Justify) -> YGJustify {
    return (scoped as YGJustify);
}
pub fn toString_150(mut e: facebook_yoga_Justify) -> Ptr<()> {
    return ({ YGJustifyToString_151(({ unscopedEnum_149(e) })) });
}
pub type facebook_yoga_Overflow = ();
pub const facebook_yoga_Overflow_Visible: facebook_yoga_Overflow = 0;
pub const facebook_yoga_Overflow_Hidden: facebook_yoga_Overflow = 1;
pub const facebook_yoga_Overflow_Scroll: facebook_yoga_Overflow = 2;
pub fn ordinalCount_152() -> () {
    return 3_();
}
pub fn scopedEnum_153(mut unscoped: YGOverflow) -> facebook_yoga_Overflow {
    return (unscoped as facebook_yoga_Overflow);
}
pub fn unscopedEnum_154(mut scoped: facebook_yoga_Overflow) -> YGOverflow {
    return (scoped as YGOverflow);
}
pub fn toString_155(mut e: facebook_yoga_Overflow) -> Ptr<()> {
    return ({ YGOverflowToString_156(({ unscopedEnum_154(e) })) });
}
pub type facebook_yoga_PositionType = ();
pub const facebook_yoga_PositionType_Static: facebook_yoga_PositionType = 0;
pub const facebook_yoga_PositionType_Relative: facebook_yoga_PositionType = 1;
pub const facebook_yoga_PositionType_Absolute: facebook_yoga_PositionType = 2;
pub fn ordinalCount_157() -> () {
    return 3_();
}
pub fn scopedEnum_158(mut unscoped: YGPositionType) -> facebook_yoga_PositionType {
    return (unscoped as facebook_yoga_PositionType);
}
pub fn unscopedEnum_159(mut scoped: facebook_yoga_PositionType) -> YGPositionType {
    return (scoped as YGPositionType);
}
pub fn toString_160(mut e: facebook_yoga_PositionType) -> Ptr<()> {
    return ({ YGPositionTypeToString_161(({ unscopedEnum_159(e) })) });
}
pub type facebook_yoga_Unit = ();
pub const facebook_yoga_Unit_Undefined: facebook_yoga_Unit = 0;
pub const facebook_yoga_Unit_Point: facebook_yoga_Unit = 1;
pub const facebook_yoga_Unit_Percent: facebook_yoga_Unit = 2;
pub const facebook_yoga_Unit_Auto: facebook_yoga_Unit = 3;
pub const facebook_yoga_Unit_MaxContent: facebook_yoga_Unit = 4;
pub const facebook_yoga_Unit_FitContent: facebook_yoga_Unit = 5;
pub const facebook_yoga_Unit_Stretch: facebook_yoga_Unit = 6;
pub fn ordinalCount_162() -> () {
    return 7_();
}
pub fn scopedEnum_163(mut unscoped: YGUnit) -> facebook_yoga_Unit {
    return (unscoped as facebook_yoga_Unit);
}
pub fn unscopedEnum_164(mut scoped: facebook_yoga_Unit) -> YGUnit {
    return (scoped as YGUnit);
}
pub fn toString_165(mut e: facebook_yoga_Unit) -> Ptr<()> {
    return ({ YGUnitToString_166(({ unscopedEnum_164(e) })) });
}
pub type facebook_yoga_Wrap = ();
pub const facebook_yoga_Wrap_NoWrap: facebook_yoga_Wrap = 0;
pub const facebook_yoga_Wrap_Wrap: facebook_yoga_Wrap = 1;
pub const facebook_yoga_Wrap_WrapReverse: facebook_yoga_Wrap = 2;
pub fn ordinalCount_167() -> () {
    return 3_();
}
pub fn scopedEnum_168(mut unscoped: YGWrap) -> facebook_yoga_Wrap {
    return (unscoped as facebook_yoga_Wrap);
}
pub fn unscopedEnum_169(mut scoped: facebook_yoga_Wrap) -> YGWrap {
    return (scoped as YGWrap);
}
pub fn toString_170(mut e: facebook_yoga_Wrap) -> Ptr<()> {
    return ({ YGWrapToString_171(({ unscopedEnum_169(e) })) });
}
pub type facebook_yoga_GridLineType = ();
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
    pub integer: (),
}
impl facebook_yoga_GridLine {
    pub fn auto_() -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Auto,
            integer: 0_(),
        };
    }
    pub fn fromInteger(mut value: ()) -> facebook_yoga_GridLine {
        return facebook_yoga_GridLine {
            type_: facebook_yoga_GridLineType_Integer,
            integer: value,
        };
    }
    pub fn span(mut value: ()) -> facebook_yoga_GridLine {
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
                integer: unimplemented!(),
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
    pub fn points(mut value: ()) -> facebook_yoga_StyleSizeLength {
        return if ({ isUndefined_99(value) }) || ({ isinf_103(value) }) {
            ({ facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Point },
            )
        };
    }
    pub fn percent(mut value: ()) -> facebook_yoga_StyleSizeLength {
        return if ({ isUndefined_99(value) }) || ({ isinf_103(value) }) {
            ({ facebook_yoga_StyleSizeLength::undefined() })
        } else {
            facebook_yoga_StyleSizeLength::new_1(
                { facebook_yoga_FloatOptional::new_1({ value }) },
                { facebook_yoga_Unit_Percent },
            )
        };
    }
    pub fn stretch(mut fraction: ()) -> facebook_yoga_StyleSizeLength {
        return if ({ isUndefined_99(fraction) }) || ({ isinf_103(fraction) }) {
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
pub fn inexactEquals_172(
    a: Ptr<facebook_yoga_StyleSizeLength>,
    b: Ptr<facebook_yoga_StyleSizeLength>,
) -> () {
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
    pub baseSize: (),
    #[offset(20)]
    pub growthLimit: (),
    #[offset(24)]
    pub infinitelyGrowable: (),
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
    pub fn length(mut points: ()) -> facebook_yoga_GridTrackSize {
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
    pub fn fr(mut fraction: ()) -> facebook_yoga_GridTrackSize {
        return facebook_yoga_GridTrackSize {
            minSizingFunction: ({ facebook_yoga_StyleSizeLength::ofAuto() }),
            maxSizingFunction: ({ facebook_yoga_StyleSizeLength::stretch(fraction) }),
            baseSize: 0_f32,
            growthLimit: 0_f32,
            infinitelyGrowable: false,
        };
    }
    pub fn percent(mut percentage: ()) -> facebook_yoga_GridTrackSize {
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
    pub templateColumns: (),
    #[offset(24)]
    #[byte_size(24)]
    pub templateRows: (),
    #[offset(48)]
    #[byte_size(24)]
    pub autoColumns: (),
    #[offset(72)]
    #[byte_size(24)]
    pub autoRows: (),
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
            templateColumns: unimplemented!(),
            templateRows: unimplemented!(),
            autoColumns: unimplemented!(),
            autoRows: unimplemented!(),
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
                templateColumns: unimplemented!(),
                templateRows: unimplemented!(),
                autoColumns: unimplemented!(),
                autoRows: unimplemented!(),
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
    grid_: (),
}
impl facebook_yoga_GridStyleStorage {
    pub fn move_from(_a0: Ptr<facebook_yoga_GridStyleStorage>) -> Self {
        Self {
            grid_: unimplemented!(),
        }
    }
    pub fn copy_from(other: Ptr<facebook_yoga_GridStyleStorage>) -> Self {
        let __this: Value<facebook_yoga_GridStyleStorage> = Rc::new(RefCell::new(Self {
            grid_: unimplemented!(),
        }));
        let this: Ptr<facebook_yoga_GridStyleStorage> = __this.as_pointer();
        ({
            let _other: Ptr<facebook_yoga_GridStyleStorage> = (other).clone();
            facebook_yoga_GridStyleStorageImpl::copy_assign(&this, _other)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn defaults() -> Ptr<facebook_yoga_GridStyle> {
        return kDefaults_173.with(|v| v.as_pointer());
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
        {
            facebook_yoga_GridStyleStorage {
                grid_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static kDefaults_173: Value<facebook_yoga_GridStyle> =
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
    pub fn points(mut value: ()) -> facebook_yoga_StyleLength {
        return if ({ isUndefined_99(value) }) || ({ isinf_103(value) }) {
            ({ facebook_yoga_StyleLength::undefined() })
        } else {
            facebook_yoga_StyleLength::new_1({ facebook_yoga_FloatOptional::new_1({ value }) }, {
                facebook_yoga_Unit_Point
            })
        };
    }
    pub fn percent(mut value: ()) -> facebook_yoga_StyleLength {
        return if ({ isUndefined_99(value) }) || ({ isinf_103(value) }) {
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
pub fn inexactEquals_174(
    a: Ptr<facebook_yoga_StyleLength>,
    b: Ptr<facebook_yoga_StyleLength>,
) -> () {
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
    pub buffer_: (),
    #[offset(24)]
    #[byte_size(40)]
    pub wideElements_: (),
}
impl facebook_yoga_SmallValueBuffer_4__Overflow {
    pub fn move_from(_a0: Ptr<facebook_yoga_SmallValueBuffer_4__Overflow>) -> Self {
        Self {
            buffer_: unimplemented!(),
            wideElements_: unimplemented!(),
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(40)]
pub struct facebook_yoga_SmallValueBuffer_4_ {
    #[offset(0)]
    count_: (),
    #[offset(4)]
    #[byte_size(16)]
    buffer_: (),
    #[offset(24)]
    #[byte_size(8)]
    wideElements_: (),
    #[offset(32)]
    #[byte_size(8)]
    overflow_: (),
}
impl facebook_yoga_SmallValueBuffer_4_ {
    pub fn copy_from(other: Ptr<facebook_yoga_SmallValueBuffer_4_>) -> Self {
        let __this: Value<facebook_yoga_SmallValueBuffer_4_> = Rc::new(RefCell::new(Self {
            count_: 0_(),
            buffer_: unimplemented!(),
            wideElements_: unimplemented!(),
            overflow_: unimplemented!(),
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
            overflow_: unimplemented!(),
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
                count_: 0_(),
                buffer_: unimplemented!(),
                wideElements_: unimplemented!(),
                overflow_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static kHandleTypeMask_175: Value<()> = Rc::new(RefCell::new(7_()));
);
thread_local!(
    static kHandleIndexedMask_176: Value<()> = Rc::new(RefCell::new(8_()));
);
thread_local!(
    static kHandleValueMask_177: Value<()> = Rc::new(RefCell::new(65520_()));
);
pub type facebook_yoga_StyleValueHandle_Type = ();
pub const facebook_yoga_StyleValueHandle_Type_Undefined: facebook_yoga_StyleValueHandle_Type = 0;
pub const facebook_yoga_StyleValueHandle_Type_Point: facebook_yoga_StyleValueHandle_Type = 1;
pub const facebook_yoga_StyleValueHandle_Type_Percent: facebook_yoga_StyleValueHandle_Type = 2;
pub const facebook_yoga_StyleValueHandle_Type_Number: facebook_yoga_StyleValueHandle_Type = 3;
pub const facebook_yoga_StyleValueHandle_Type_Auto: facebook_yoga_StyleValueHandle_Type = 4;
pub const facebook_yoga_StyleValueHandle_Type_Keyword: facebook_yoga_StyleValueHandle_Type = 5;
pub type facebook_yoga_StyleValueHandle_Keyword = ();
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
    repr_: (),
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
        { facebook_yoga_StyleValueHandle { repr_: 0_() } }
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
    fn isIntegerPackable(mut f: ()) -> () {
        let mut kMaxInlineAbsValue: () = ((1_() << 11_()) - 1_());
        let mut i: () = f;
        return ((i == f) && (i >= -kMaxInlineAbsValue)) && (i <= kMaxInlineAbsValue);
    }
    fn packInlineInteger(mut value: ()) -> () {
        let mut isNegative: () = if (value < 0_()) { 1_() } else { 0_() };
        return ((isNegative << 11_())
            | (value * (if (isNegative != 0_()) { -1_() } else { 1_() })));
    }
    fn unpackInlineInteger(mut value: ()) -> () {
        let mut kValueSignMask: () = 2048_();
        let mut kValueMagnitudeMask: () = 2047_();
        let mut isNegative: () = ((value & kValueSignMask) != 0_());
        return ((value & kValueMagnitudeMask) * (if isNegative { -1_() } else { 1_() }));
    }
    pub fn move_from(_a0: Ptr<facebook_yoga_StyleValuePool>) -> Self {
        Self {
            buffer_: facebook_yoga_SmallValueBuffer_4_::move_from({ field_ptr!(_a0, buffer_) }),
        }
    }
}
thread_local!(
    pub static DefaultFlexGrow_178: Value<()> = Rc::new(RefCell::new(0_f32));
);
thread_local!(
    pub static DefaultFlexShrink_179: Value<()> = Rc::new(RefCell::new(0_f32));
);
thread_local!(
    pub static WebDefaultFlexShrink_180: Value<()> = Rc::new(RefCell::new(1_f32));
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
    margin_: (),
    #[offset(31)]
    #[byte_size(18)]
    position_: (),
    #[offset(49)]
    #[byte_size(18)]
    padding_: (),
    #[offset(67)]
    #[byte_size(18)]
    border_: (),
    #[offset(85)]
    #[byte_size(6)]
    gap_: (),
    #[offset(91)]
    #[byte_size(4)]
    dimensions_: (),
    #[offset(95)]
    #[byte_size(4)]
    minDimensions_: (),
    #[offset(99)]
    #[byte_size(4)]
    maxDimensions_: (),
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
    ) -> () {
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
                operator_eq_108(_lhs, _rhs)
            });
    }
    fn lengthsEqual(
        lhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhsHandle: Ptr<facebook_yoga_StyleValueHandle>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> () {
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
    ) -> () {
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
    fn anyIsPercent_121_9(handles: Ptr<()>) -> () {
        return unimplemented!();
    }
    fn lengthsEqual_124_9(
        lhs: Ptr<()>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhs: Ptr<()>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> () {
        return unimplemented!();
    }
    fn lengthsEqual_124_3(
        lhs: Ptr<()>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhs: Ptr<()>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> () {
        return unimplemented!();
    }
    fn sizeLengthsEqual_126_2(
        lhs: Ptr<()>,
        lhsPool: Ptr<facebook_yoga_StyleValuePool>,
        rhs: Ptr<()>,
        rhsPool: Ptr<facebook_yoga_StyleValuePool>,
    ) -> () {
        return unimplemented!();
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
                margin_: unimplemented!(),
                position_: unimplemented!(),
                padding_: unimplemented!(),
                border_: unimplemented!(),
                gap_: unimplemented!(),
                dimensions_: unimplemented!(),
                minDimensions_: unimplemented!(),
                maxDimensions_: unimplemented!(),
                aspectRatio_: <facebook_yoga_StyleValueHandle>::default(),
                grid_: <facebook_yoga_GridStyleStorage>::default(),
                pool_: <facebook_yoga_StyleValuePool>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct YGNode {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(720)]
pub struct facebook_yoga_Node {
    #[offset(0)]
    hasNewLayout_: (),
    #[offset(0)]
    isReferenceBaseline_: (),
    #[offset(0)]
    isDirty_: (),
    #[offset(0)]
    alwaysFormsContainingBlock_: (),
    #[offset(0)]
    #[byte_size(1)]
    nodeType_: facebook_yoga_NodeType,
    #[offset(8)]
    #[byte_size(8)]
    context_: (),
    #[offset(16)]
    #[byte_size(8)]
    measureFunc_: FnPtr<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
    #[offset(24)]
    #[byte_size(8)]
    minContentMeasureFunc_: FnPtr<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
    #[offset(32)]
    #[byte_size(4)]
    minContentWidth_: facebook_yoga_FloatOptional,
    #[offset(36)]
    #[byte_size(4)]
    minContentHeight_: facebook_yoga_FloatOptional,
    #[offset(40)]
    #[byte_size(8)]
    baselineFunc_: FnPtr<fn(Ptr<YGNode>, (), ()) -> ()>,
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
    lineIndex_: (),
    #[offset(656)]
    contentsChildrenCount_: (),
    #[offset(664)]
    #[byte_size(8)]
    owner_: Ptr<facebook_yoga_Node>,
    #[offset(672)]
    #[byte_size(24)]
    children_: (),
    #[offset(696)]
    #[byte_size(8)]
    config_: Ptr<facebook_yoga_Config>,
    #[offset(704)]
    #[byte_size(16)]
    processedDimensions_: (),
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
                context_: unimplemented!(),
                measureFunc_: FnPtr::<
                    fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
                >::null(),
                minContentMeasureFunc_: FnPtr::<
                    fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
                >::null(),
                minContentWidth_: <facebook_yoga_FloatOptional>::default(),
                minContentHeight_: <facebook_yoga_FloatOptional>::default(),
                baselineFunc_: FnPtr::<fn(Ptr<YGNode>, (), ()) -> ()>::null(),
                dirtiedFunc_: FnPtr::<fn(Ptr<YGNode>)>::null(),
                style_: <facebook_yoga_Style>::default(),
                layout_: <facebook_yoga_LayoutResults>::default(),
                lineIndex_: 0_(),
                contentsChildrenCount_: 0_(),
                owner_: Ptr::<facebook_yoga_Node>::null(),
                children_: unimplemented!(),
                config_: Ptr::<facebook_yoga_Config>::null(),
                processedDimensions_: unimplemented!(),
            }
        }
    }
}
pub fn resolveRef_181(mut ref_: Ptr<YGNode>) -> Ptr<facebook_yoga_Node> {
    return ref_.reinterpret_cast::<facebook_yoga_Node>();
}
pub fn resolveRef_182(mut ref_: Ptr<YGNode>) -> Ptr<facebook_yoga_Node> {
    return ref_.reinterpret_cast::<facebook_yoga_Node>();
}
pub fn YGConfigNew_183() -> Ptr<YGConfig> {
    return Ptr::alloc(facebook_yoga_Config::new({ ({ getDefaultLogger_184() }) }));
}
pub fn YGConfigFree_185(mut config: Ptr<YGConfig>) {
    ({ resolveRef_63((config).clone()) }).delete();
}
pub fn YGConfigGetDefault_186() -> Ptr<YGConfig> {
    return ({ facebook_yoga_Config::getDefault() });
}
pub fn YGConfigSetUseWebDefaults_187(mut config: Ptr<YGConfig>, mut enabled: ()) {
    ({
        facebook_yoga_ConfigImpl::setUseWebDefaults(&({ resolveRef_63((config).clone()) }), enabled)
    });
}
pub fn YGConfigGetUseWebDefaults_188(mut config: Ptr<YGConfig>) -> () {
    return ({ facebook_yoga_ConfigImpl::useWebDefaults(&({ resolveRef_64(config) })) });
}
pub fn YGConfigSetPointScaleFactor_189(mut config: Ptr<YGConfig>, mut pixelsInPoint: ()) {
    ({
        assertFatalWithConfig_190(
            ({ resolveRef_63((config).clone()) }),
            (pixelsInPoint >= 0_f32),
            Ptr::<()>::from_string_literal(b"Scale factor should not be less than zero"),
        )
    });
    ({
        facebook_yoga_ConfigImpl::setPointScaleFactor(
            &({ resolveRef_63((config).clone()) }),
            pixelsInPoint,
        )
    });
}
pub fn YGConfigGetPointScaleFactor_191(mut config: Ptr<YGConfig>) -> () {
    return ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&({ resolveRef_64(config) })) });
}
pub fn YGConfigSetErrata_192(mut config: Ptr<YGConfig>, mut errata: YGErrata) {
    ({
        facebook_yoga_ConfigImpl::setErrata(
            &({ resolveRef_63((config).clone()) }),
            ({ scopedEnum_48(errata) }),
        )
    });
}
pub fn YGConfigGetErrata_193(mut config: Ptr<YGConfig>) -> YGErrata {
    return ({
        unscopedEnum_49(({ facebook_yoga_ConfigImpl::getErrata(&({ resolveRef_64(config) })) }))
    });
}
pub fn YGConfigSetLogger_194(
    mut config: Ptr<YGConfig>,
    mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>,
) {
    if !((logger).is_null()) {
        ({
            facebook_yoga_ConfigImpl::setLogger(
                &({ resolveRef_63((config).clone()) }),
                (logger).clone(),
            )
        });
    } else {
        ({
            facebook_yoga_ConfigImpl::setLogger(
                &({ resolveRef_63((config).clone()) }),
                ({ getDefaultLogger_184() }),
            )
        });
    }
}
pub fn YGConfigSetContext_195(mut config: Ptr<YGConfig>, mut context: ()) {
    ({
        facebook_yoga_ConfigImpl::setContext(
            &({ resolveRef_63((config).clone()) }),
            (context).clone(),
        )
    });
}
pub fn YGConfigGetContext_196(mut config: Ptr<YGConfig>) -> () {
    return ({ facebook_yoga_ConfigImpl::getContext(&({ resolveRef_64(config) })) });
}
pub fn YGConfigSetExperimentalFeatureEnabled_197(
    mut config: Ptr<YGConfig>,
    mut feature: YGExperimentalFeature,
    mut enabled: (),
) {
    ({
        facebook_yoga_ConfigImpl::setExperimentalFeatureEnabled(
            &({ resolveRef_63((config).clone()) }),
            ({ scopedEnum_53(feature) }),
            enabled,
        )
    });
}
pub fn YGConfigIsExperimentalFeatureEnabled_198(
    mut config: Ptr<YGConfig>,
    mut feature: YGExperimentalFeature,
) -> () {
    return ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ resolveRef_64(config) }),
            ({ scopedEnum_53(feature) }),
        )
    });
}
pub fn YGConfigSetCloneNodeFunc_199(
    mut config: Ptr<YGConfig>,
    mut callback: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, ()) -> Ptr<YGNode>>,
) {
    ({
        facebook_yoga_ConfigImpl::setCloneNodeCallback(
            &({ resolveRef_63((config).clone()) }),
            (callback).clone(),
        )
    });
}
pub fn YGAlignToString_136(mut value: YGAlign) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGAlign_YGAlignAuto as ()) => {
                return Ptr::<()>::from_string_literal(b"auto");
            }
            __v if __v == (YGAlign_YGAlignFlexStart as ()) => {
                return Ptr::<()>::from_string_literal(b"flex-start");
            }
            __v if __v == (YGAlign_YGAlignCenter as ()) => {
                return Ptr::<()>::from_string_literal(b"center");
            }
            __v if __v == (YGAlign_YGAlignFlexEnd as ()) => {
                return Ptr::<()>::from_string_literal(b"flex-end");
            }
            __v if __v == (YGAlign_YGAlignStretch as ()) => {
                return Ptr::<()>::from_string_literal(b"stretch");
            }
            __v if __v == (YGAlign_YGAlignBaseline as ()) => {
                return Ptr::<()>::from_string_literal(b"baseline");
            }
            __v if __v == (YGAlign_YGAlignSpaceBetween as ()) => {
                return Ptr::<()>::from_string_literal(b"space-between");
            }
            __v if __v == (YGAlign_YGAlignSpaceAround as ()) => {
                return Ptr::<()>::from_string_literal(b"space-around");
            }
            __v if __v == (YGAlign_YGAlignSpaceEvenly as ()) => {
                return Ptr::<()>::from_string_literal(b"space-evenly");
            }
            __v if __v == (YGAlign_YGAlignStart as ()) => {
                return Ptr::<()>::from_string_literal(b"start");
            }
            __v if __v == (YGAlign_YGAlignEnd as ()) => {
                return Ptr::<()>::from_string_literal(b"end");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGBoxSizingToString_141(mut value: YGBoxSizing) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGBoxSizing_YGBoxSizingBorderBox as ()) => {
                return Ptr::<()>::from_string_literal(b"border-box");
            }
            __v if __v == (YGBoxSizing_YGBoxSizingContentBox as ()) => {
                return Ptr::<()>::from_string_literal(b"content-box");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGDimensionToString_75(mut value: YGDimension) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGDimension_YGDimensionWidth as ()) => {
                return Ptr::<()>::from_string_literal(b"width");
            }
            __v if __v == (YGDimension_YGDimensionHeight as ()) => {
                return Ptr::<()>::from_string_literal(b"height");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGDirectionToString_80(mut value: YGDirection) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGDirection_YGDirectionInherit as ()) => {
                return Ptr::<()>::from_string_literal(b"inherit");
            }
            __v if __v == (YGDirection_YGDirectionLTR as ()) => {
                return Ptr::<()>::from_string_literal(b"ltr");
            }
            __v if __v == (YGDirection_YGDirectionRTL as ()) => {
                return Ptr::<()>::from_string_literal(b"rtl");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGDisplayToString_69(mut value: YGDisplay) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGDisplay_YGDisplayFlex as ()) => {
                return Ptr::<()>::from_string_literal(b"flex");
            }
            __v if __v == (YGDisplay_YGDisplayNone as ()) => {
                return Ptr::<()>::from_string_literal(b"none");
            }
            __v if __v == (YGDisplay_YGDisplayContents as ()) => {
                return Ptr::<()>::from_string_literal(b"contents");
            }
            __v if __v == (YGDisplay_YGDisplayGrid as ()) => {
                return Ptr::<()>::from_string_literal(b"grid");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGEdgeToString_85(mut value: YGEdge) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGEdge_YGEdgeLeft as ()) => {
                return Ptr::<()>::from_string_literal(b"left");
            }
            __v if __v == (YGEdge_YGEdgeTop as ()) => {
                return Ptr::<()>::from_string_literal(b"top");
            }
            __v if __v == (YGEdge_YGEdgeRight as ()) => {
                return Ptr::<()>::from_string_literal(b"right");
            }
            __v if __v == (YGEdge_YGEdgeBottom as ()) => {
                return Ptr::<()>::from_string_literal(b"bottom");
            }
            __v if __v == (YGEdge_YGEdgeStart as ()) => {
                return Ptr::<()>::from_string_literal(b"start");
            }
            __v if __v == (YGEdge_YGEdgeEnd as ()) => {
                return Ptr::<()>::from_string_literal(b"end");
            }
            __v if __v == (YGEdge_YGEdgeHorizontal as ()) => {
                return Ptr::<()>::from_string_literal(b"horizontal");
            }
            __v if __v == (YGEdge_YGEdgeVertical as ()) => {
                return Ptr::<()>::from_string_literal(b"vertical");
            }
            __v if __v == (YGEdge_YGEdgeAll as ()) => {
                return Ptr::<()>::from_string_literal(b"all");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGErrataToString_51(mut value: YGErrata) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGErrata_YGErrataNone as ()) => {
                return Ptr::<()>::from_string_literal(b"none");
            }
            __v if __v == (YGErrata_YGErrataStretchFlexBasis as ()) => {
                return Ptr::<()>::from_string_literal(b"stretch-flex-basis");
            }
            __v if __v == (YGErrata_YGErrataAbsolutePositionWithoutInsetsExcludesPadding as ()) => {
                return Ptr::<()>::from_string_literal(
                    b"absolute-position-without-insets-excludes-padding",
                );
            }
            __v if __v == (YGErrata_YGErrataAbsolutePercentAgainstInnerSize as ()) => {
                return Ptr::<()>::from_string_literal(b"absolute-percent-against-inner-size");
            }
            __v if __v == (YGErrata_YGErrataMinSizeUndefinedInsteadOfAuto as ()) => {
                return Ptr::<()>::from_string_literal(b"min-size-undefined-instead-of-auto");
            }
            __v if __v == (YGErrata_YGErrataFlexFirstPassUsesRunningTotals as ()) => {
                return Ptr::<()>::from_string_literal(b"flex-first-pass-uses-running-totals");
            }
            __v if __v == (YGErrata_YGErrataAll as ()) => {
                return Ptr::<()>::from_string_literal(b"all");
            }
            __v if __v == (YGErrata_YGErrataClassic as ()) => {
                return Ptr::<()>::from_string_literal(b"classic");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGExperimentalFeatureToString_56(mut value: YGExperimentalFeature) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGExperimentalFeature_YGExperimentalFeatureWebFlexBasis as ()) => {
                return Ptr::<()>::from_string_literal(b"web-flex-basis");
            }
            __v if __v
                == (YGExperimentalFeature_YGExperimentalFeatureFixFlexBasisFitContent as ()) =>
            {
                return Ptr::<()>::from_string_literal(b"fix-flex-basis-fit-content");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGFlexDirectionToString_122(mut value: YGFlexDirection) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGFlexDirection_YGFlexDirectionColumn as ()) => {
                return Ptr::<()>::from_string_literal(b"column");
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionColumnReverse as ()) => {
                return Ptr::<()>::from_string_literal(b"column-reverse");
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRow as ()) => {
                return Ptr::<()>::from_string_literal(b"row");
            }
            __v if __v == (YGFlexDirection_YGFlexDirectionRowReverse as ()) => {
                return Ptr::<()>::from_string_literal(b"row-reverse");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGGridTrackTypeToString_200(mut value: YGGridTrackType) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as ()) => {
                return Ptr::<()>::from_string_literal(b"auto");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as ()) => {
                return Ptr::<()>::from_string_literal(b"points");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as ()) => {
                return Ptr::<()>::from_string_literal(b"percent");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as ()) => {
                return Ptr::<()>::from_string_literal(b"fr");
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as ()) => {
                return Ptr::<()>::from_string_literal(b"minmax");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGGutterToString_146(mut value: YGGutter) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGGutter_YGGutterColumn as ()) => {
                return Ptr::<()>::from_string_literal(b"column");
            }
            __v if __v == (YGGutter_YGGutterRow as ()) => {
                return Ptr::<()>::from_string_literal(b"row");
            }
            __v if __v == (YGGutter_YGGutterAll as ()) => {
                return Ptr::<()>::from_string_literal(b"all");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGJustifyToString_151(mut value: YGJustify) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGJustify_YGJustifyAuto as ()) => {
                return Ptr::<()>::from_string_literal(b"auto");
            }
            __v if __v == (YGJustify_YGJustifyFlexStart as ()) => {
                return Ptr::<()>::from_string_literal(b"flex-start");
            }
            __v if __v == (YGJustify_YGJustifyCenter as ()) => {
                return Ptr::<()>::from_string_literal(b"center");
            }
            __v if __v == (YGJustify_YGJustifyFlexEnd as ()) => {
                return Ptr::<()>::from_string_literal(b"flex-end");
            }
            __v if __v == (YGJustify_YGJustifySpaceBetween as ()) => {
                return Ptr::<()>::from_string_literal(b"space-between");
            }
            __v if __v == (YGJustify_YGJustifySpaceAround as ()) => {
                return Ptr::<()>::from_string_literal(b"space-around");
            }
            __v if __v == (YGJustify_YGJustifySpaceEvenly as ()) => {
                return Ptr::<()>::from_string_literal(b"space-evenly");
            }
            __v if __v == (YGJustify_YGJustifyStretch as ()) => {
                return Ptr::<()>::from_string_literal(b"stretch");
            }
            __v if __v == (YGJustify_YGJustifyStart as ()) => {
                return Ptr::<()>::from_string_literal(b"start");
            }
            __v if __v == (YGJustify_YGJustifyEnd as ()) => {
                return Ptr::<()>::from_string_literal(b"end");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGLogLevelToString_61(mut value: YGLogLevel) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGLogLevel_YGLogLevelError as ()) => {
                return Ptr::<()>::from_string_literal(b"error");
            }
            __v if __v == (YGLogLevel_YGLogLevelWarn as ()) => {
                return Ptr::<()>::from_string_literal(b"warn");
            }
            __v if __v == (YGLogLevel_YGLogLevelInfo as ()) => {
                return Ptr::<()>::from_string_literal(b"info");
            }
            __v if __v == (YGLogLevel_YGLogLevelDebug as ()) => {
                return Ptr::<()>::from_string_literal(b"debug");
            }
            __v if __v == (YGLogLevel_YGLogLevelVerbose as ()) => {
                return Ptr::<()>::from_string_literal(b"verbose");
            }
            __v if __v == (YGLogLevel_YGLogLevelFatal as ()) => {
                return Ptr::<()>::from_string_literal(b"fatal");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGMeasureModeToString_90(mut value: YGMeasureMode) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGMeasureMode_YGMeasureModeUndefined as ()) => {
                return Ptr::<()>::from_string_literal(b"undefined");
            }
            __v if __v == (YGMeasureMode_YGMeasureModeExactly as ()) => {
                return Ptr::<()>::from_string_literal(b"exactly");
            }
            __v if __v == (YGMeasureMode_YGMeasureModeAtMost as ()) => {
                return Ptr::<()>::from_string_literal(b"at-most");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGNodeTypeToString_95(mut value: YGNodeType) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGNodeType_YGNodeTypeDefault as ()) => {
                return Ptr::<()>::from_string_literal(b"default");
            }
            __v if __v == (YGNodeType_YGNodeTypeText as ()) => {
                return Ptr::<()>::from_string_literal(b"text");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGOverflowToString_156(mut value: YGOverflow) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGOverflow_YGOverflowVisible as ()) => {
                return Ptr::<()>::from_string_literal(b"visible");
            }
            __v if __v == (YGOverflow_YGOverflowHidden as ()) => {
                return Ptr::<()>::from_string_literal(b"hidden");
            }
            __v if __v == (YGOverflow_YGOverflowScroll as ()) => {
                return Ptr::<()>::from_string_literal(b"scroll");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGPositionTypeToString_161(mut value: YGPositionType) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGPositionType_YGPositionTypeStatic as ()) => {
                return Ptr::<()>::from_string_literal(b"static");
            }
            __v if __v == (YGPositionType_YGPositionTypeRelative as ()) => {
                return Ptr::<()>::from_string_literal(b"relative");
            }
            __v if __v == (YGPositionType_YGPositionTypeAbsolute as ()) => {
                return Ptr::<()>::from_string_literal(b"absolute");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGUnitToString_166(mut value: YGUnit) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGUnit_YGUnitUndefined as ()) => {
                return Ptr::<()>::from_string_literal(b"undefined");
            }
            __v if __v == (YGUnit_YGUnitPoint as ()) => {
                return Ptr::<()>::from_string_literal(b"point");
            }
            __v if __v == (YGUnit_YGUnitPercent as ()) => {
                return Ptr::<()>::from_string_literal(b"percent");
            }
            __v if __v == (YGUnit_YGUnitAuto as ()) => {
                return Ptr::<()>::from_string_literal(b"auto");
            }
            __v if __v == (YGUnit_YGUnitMaxContent as ()) => {
                return Ptr::<()>::from_string_literal(b"max-content");
            }
            __v if __v == (YGUnit_YGUnitFitContent as ()) => {
                return Ptr::<()>::from_string_literal(b"fit-content");
            }
            __v if __v == (YGUnit_YGUnitStretch as ()) => {
                return Ptr::<()>::from_string_literal(b"stretch");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub fn YGWrapToString_171(mut value: YGWrap) -> Ptr<()> {
    'switch: {
        match { (value as ()) } {
            __v if __v == (YGWrap_YGWrapNoWrap as ()) => {
                return Ptr::<()>::from_string_literal(b"no-wrap");
            }
            __v if __v == (YGWrap_YGWrapWrap as ()) => {
                return Ptr::<()>::from_string_literal(b"wrap");
            }
            __v if __v == (YGWrap_YGWrapWrapReverse as ()) => {
                return Ptr::<()>::from_string_literal(b"wrap-reverse");
            }
            _ => {}
        }
    };
    return Ptr::<()>::from_string_literal(b"unknown");
}
pub type facebook_yoga_LayoutType = ();
pub const facebook_yoga_LayoutType_kLayout: facebook_yoga_LayoutType = 0;
pub const facebook_yoga_LayoutType_kMeasure: facebook_yoga_LayoutType = 1;
pub const facebook_yoga_LayoutType_kCachedLayout: facebook_yoga_LayoutType = 2;
pub const facebook_yoga_LayoutType_kCachedMeasure: facebook_yoga_LayoutType = 3;
pub type facebook_yoga_LayoutPassReason = ();
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
    pub layouts: (),
    #[offset(4)]
    pub measures: (),
    #[offset(8)]
    pub maxMeasureCache: (),
    #[offset(12)]
    pub cachedLayouts: (),
    #[offset(16)]
    pub cachedMeasures: (),
    #[offset(20)]
    pub measureCallbacks: (),
    #[offset(24)]
    #[byte_size(36)]
    pub measureCallbackReasonsCount: (),
}
impl Default for facebook_yoga_LayoutData {
    fn default() -> Self {
        {
            facebook_yoga_LayoutData {
                layouts: 0_(),
                measures: 0_(),
                maxMeasureCache: 0_(),
                cachedLayouts: 0_(),
                cachedMeasures: 0_(),
                measureCallbacks: 0_(),
                measureCallbackReasonsCount: unimplemented!(),
            }
        }
    }
}
pub type facebook_yoga_Event_Type = ();
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
    pub width: (),
    #[offset(4)]
    pub widthMeasureMode: YGMeasureMode,
    #[offset(8)]
    pub height: (),
    #[offset(12)]
    pub heightMeasureMode: YGMeasureMode,
    #[offset(16)]
    pub measuredWidth: (),
    #[offset(20)]
    pub measuredHeight: (),
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
    data_: (),
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
                ((0_()) as facebook_yoga_Event_Type),
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
                ((1_()) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
}
pub fn YGNodeNew_7() -> Ptr<YGNode> {
    return ({ YGNodeNewWithConfig_201(({ YGConfigGetDefault_186() })) });
}
pub fn YGNodeNewWithConfig_201(mut config: Ptr<YGConfig>) -> Ptr<YGNode> {
    let mut node: Ptr<facebook_yoga_Node> = Ptr::alloc(facebook_yoga_Node::new_1({
        ({ resolveRef_64((config).clone()) })
    }));
    ({
        assertFatal_202(
            !((config).is_null()),
            Ptr::<()>::from_string_literal(b"Tried to construct YGNode with null config"),
        )
    });
    ({
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
                    config: (config).clone(),
                },
            ));
        facebook_yoga_Event::publish_3_0((node).clone(), _eventData.as_pointer())
    });
    return node;
}
pub fn YGNodeClone_203(mut oldNodeRef: Ptr<YGNode>) -> Ptr<YGNode> {
    let mut oldNode: Ptr<facebook_yoga_Node> = ({ resolveRef_182((oldNodeRef).clone()) });
    let mut node: Ptr<facebook_yoga_Node> = Ptr::alloc((*oldNode.upgrade().deref()).clone());
    ({
        let _node: Ptr<YGNode> = (node).clone();
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeAllocation_ {
                    config: ({ facebook_yoga_NodeImpl::getConfig(&node) }),
                },
            ));
        facebook_yoga_Event::publish_3_0(_node, _eventData.as_pointer())
    });
    ({ facebook_yoga_NodeImpl::setOwner(&node, Ptr::<facebook_yoga_Node>::null()) });
    return node;
}
pub fn YGNodeFree_204(mut nodeRef: Ptr<YGNode>) {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    if !(owner).is_null() {
        ({ facebook_yoga_NodeImpl::removeChild_78(&owner, (node).clone()) });
        ({ facebook_yoga_NodeImpl::setOwner(&node, Ptr::<facebook_yoga_Node>::null()) });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
    }
    let mut childCount: () = ({ facebook_yoga_NodeImpl::getChildCount(&node) });
    let mut i: () = 0_();
    'loop_: while (i < childCount) {
        let mut child: Ptr<facebook_yoga_Node> = ({ facebook_yoga_NodeImpl::getChild(&node, i) });
        ({ facebook_yoga_NodeImpl::setOwner(&child, Ptr::<facebook_yoga_Node>::null()) });
        i.postfix_inc();
    }
    ({ facebook_yoga_NodeImpl::clearChildren(&node) });
    ({
        let _node: Ptr<YGNode> = (node).clone();
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                    config: ({ YGNodeGetConfig_205((node).clone()) }),
                },
            ));
        facebook_yoga_Event::publish_3_1(_node, _eventData.as_pointer())
    });
    ({ resolveRef_181((node).clone()) }).delete();
}
pub fn YGNodeFreeRecursive_14(mut rootRef: Ptr<YGNode>) {
    let mut root: Ptr<facebook_yoga_Node> = ({ resolveRef_181((rootRef).clone()) });
    let mut skipped: () = 0_();
    'loop_: while ({ ({ facebook_yoga_NodeImpl::getChildCount(&root) }) } > { skipped }) {
        let mut child: Ptr<facebook_yoga_Node> =
            ({ facebook_yoga_NodeImpl::getChild(&root, skipped) });
        if ({ ({ facebook_yoga_NodeImpl::getOwner(&child) }) } != { (root).clone() }) {
            skipped = { (skipped).wrapping_add(1_()) };
        } else {
            ({ YGNodeRemoveChild_206((root).clone(), (child).clone()) });
            ({ YGNodeFreeRecursive_14((child).clone()) });
        }
    }
    ({ YGNodeFree_204((root).clone()) });
}
pub fn YGNodeFinalize_207(mut node: Ptr<YGNode>) {
    ({
        let _node: Ptr<YGNode> = (node).clone();
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeDeallocation_ {
                    config: ({ YGNodeGetConfig_205((node).clone()) }),
                },
            ));
        facebook_yoga_Event::publish_3_1(_node, _eventData.as_pointer())
    });
    ({ resolveRef_181((node).clone()) }).delete();
}
pub fn YGNodeReset_208(mut node: Ptr<YGNode>) {
    ({ facebook_yoga_NodeImpl::reset(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeCalculateLayout_13(
    mut node: Ptr<YGNode>,
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut ownerDirection: YGDirection,
) {
    ({
        calculateLayout_209(
            ({ resolveRef_181((node).clone()) }),
            ownerWidth,
            ownerHeight,
            ({ scopedEnum_77(ownerDirection) }),
        )
    });
}
pub fn YGNodeGetHasNewLayout_210(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::getHasNewLayout(&({ resolveRef_182(node) })) });
}
pub fn YGNodeSetHasNewLayout_211(mut node: Ptr<YGNode>, mut hasNewLayout: ()) {
    ({
        facebook_yoga_NodeImpl::setHasNewLayout(&({ resolveRef_181((node).clone()) }), hasNewLayout)
    });
}
pub fn YGNodeIsDirty_212(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::isDirty(&({ resolveRef_182(node) })) });
}
pub fn YGNodeMarkDirty_213(mut nodeRef: Ptr<YGNode>) {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _condition: () = ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) });
        assertFatalWithNode_214 ( _node , _condition , Ptr::<() >::from_string_literal(b"Only leaf nodes with custom measure functions should manually mark themselves as dirty" )  , )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&node) });
}
pub fn YGNodeSetDirtiedFunc_215(mut node: Ptr<YGNode>, mut dirtiedFunc: FnPtr<fn(Ptr<YGNode>)>) {
    ({
        facebook_yoga_NodeImpl::setDirtiedFunc(
            &({ resolveRef_181((node).clone()) }),
            (dirtiedFunc).clone(),
        )
    });
}
pub fn YGNodeGetDirtiedFunc_216(mut node: Ptr<YGNode>) -> FnPtr<fn(Ptr<YGNode>)> {
    return ({ facebook_yoga_NodeImpl::getDirtiedFunc(&({ resolveRef_182(node) })) });
}
pub fn YGNodeInsertChild_12(mut ownerRef: Ptr<YGNode>, mut childRef: Ptr<YGNode>, mut index: ()) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_181((ownerRef).clone()) });
    let mut child: Ptr<facebook_yoga_Node> = ({ resolveRef_181((childRef).clone()) });
    ({
        assertFatalWithNode_214(
            (owner).clone(),
            ({ facebook_yoga_NodeImpl::getOwner(&child) }).is_null(),
            Ptr::<()>::from_string_literal(b"Child already has a owner, it must be removed first."),
        )
    });
    ({
        let _node: Ptr<facebook_yoga_Node> = (owner).clone();
        let _condition: () = !({ facebook_yoga_NodeImpl::hasMeasureFunc(&owner) });
        assertFatalWithNode_214(
            _node,
            _condition,
            Ptr::<()>::from_string_literal(
                b"Cannot add child: Nodes with measure functions cannot have children.",
            ),
        )
    });
    ({ facebook_yoga_NodeImpl::insertChild(&owner, (child).clone(), index) });
    ({ facebook_yoga_NodeImpl::setOwner(&child, (owner).clone()) });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
}
pub fn YGNodeSwapChild_217(mut ownerRef: Ptr<YGNode>, mut childRef: Ptr<YGNode>, mut index: ()) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_181((ownerRef).clone()) });
    let mut child: Ptr<facebook_yoga_Node> = ({ resolveRef_181((childRef).clone()) });
    ({ facebook_yoga_NodeImpl::replaceChild_76(&owner, (child).clone(), index) });
    ({ facebook_yoga_NodeImpl::setOwner(&child, (owner).clone()) });
}
pub fn YGNodeRemoveChild_206(mut ownerRef: Ptr<YGNode>, mut excludedChildRef: Ptr<YGNode>) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_181((ownerRef).clone()) });
    let mut excludedChild: Ptr<facebook_yoga_Node> =
        ({ resolveRef_181((excludedChildRef).clone()) });
    if (({ facebook_yoga_NodeImpl::getChildCount(&owner) }) == 0_()) {
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
pub fn YGNodeRemoveAllChildren_218(mut ownerRef: Ptr<YGNode>) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_181((ownerRef).clone()) });
    let mut childCount: () = ({ facebook_yoga_NodeImpl::getChildCount(&owner) });
    if (childCount == 0_()) {
        return;
    }
    let mut firstChild: Ptr<facebook_yoga_Node> =
        ({ facebook_yoga_NodeImpl::getChild(&owner, 0_()) });
    if ({ ({ facebook_yoga_NodeImpl::getOwner(&firstChild) }) } == { (owner).clone() }) {
        let mut i: () = 0_();
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
        let _children: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        facebook_yoga_NodeImpl::setChildren(&owner, _children.as_pointer())
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
}
pub fn YGNodeSetChildren_219(
    mut ownerRef: Ptr<YGNode>,
    mut childrenRefs: Ptr<Ptr<YGNode>>,
    mut count: (),
) {
    let mut owner: Ptr<facebook_yoga_Node> = ({ resolveRef_181((ownerRef).clone()) });
    let mut children: Ptr<Ptr<facebook_yoga_Node>> =
        childrenRefs.reinterpret_cast::<*mut facebook_yoga_Node>();
    if (owner).is_null() {
        return;
    }
    let childrenVector: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if unimplemented!() {
        if (({ facebook_yoga_NodeImpl::getChildCount(&owner) }) > 0_()) {
            let __range3: Ptr<()> = ({ facebook_yoga_NodeImpl::getChildren(&owner) });
            'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
                ({
                    let _layout: Value<facebook_yoga_LayoutResults> =
                        Rc::new(RefCell::new(<facebook_yoga_LayoutResults>::default()));
                    facebook_yoga_NodeImpl::setLayout(&child, _layout.as_pointer())
                });
                ({ facebook_yoga_NodeImpl::setOwner(&child, Ptr::<facebook_yoga_Node>::null()) });
            }
            ({
                let _children: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                facebook_yoga_NodeImpl::setChildren(&owner, _children.as_pointer())
            });
            ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
        }
    } else {
        if (({ facebook_yoga_NodeImpl::getChildCount(&owner) }) > 0_()) {
            let __range3: Ptr<()> = ({ facebook_yoga_NodeImpl::getChildren(&owner) });
            'loop_: for oldChild in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut oldChild: Ptr<facebook_yoga_Node> = (unimplemented!().read());
                if unimplemented!() {
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
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
            ({ facebook_yoga_NodeImpl::setOwner(&child, (owner).clone()) });
        }
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&owner) });
    }
}
pub fn YGNodeGetChild_220(mut nodeRef: Ptr<YGNode>, mut index: ()) -> Ptr<YGNode> {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    if ({ index } < { unimplemented!() }) {
        return ({ facebook_yoga_NodeImpl::getChild(&node, index) });
    }
    return Ptr::<YGNode>::null();
}
pub fn YGNodeGetChildCount_221(mut node: Ptr<YGNode>) -> () {
    return unimplemented!();
}
pub fn YGNodeGetOwner_222(mut node: Ptr<YGNode>) -> Ptr<YGNode> {
    return ({ facebook_yoga_NodeImpl::getOwner(&({ resolveRef_181(node) })) });
}
pub fn YGNodeGetParent_223(mut node: Ptr<YGNode>) -> Ptr<YGNode> {
    return ({ facebook_yoga_NodeImpl::getOwner(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetConfig_224(mut node: Ptr<YGNode>, mut config: Ptr<YGConfig>) {
    ({
        facebook_yoga_NodeImpl::setConfig(
            &({ resolveRef_181((node).clone()) }),
            ({ resolveRef_63((config).clone()) }),
        )
    });
}
pub fn YGNodeGetConfig_205(mut node: Ptr<YGNode>) -> Ptr<YGConfig> {
    return ({ facebook_yoga_NodeImpl::getConfig(&({ resolveRef_181(node) })) });
}
pub fn YGNodeSetContext_225(mut node: Ptr<YGNode>, mut context: ()) {
    ({ facebook_yoga_NodeImpl::setContext(&({ resolveRef_181(node) }), context) });
    return;
}
pub fn YGNodeGetContext_226(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::getContext(&({ resolveRef_182(node) })) });
}
pub fn YGNodeSetMeasureFunc_10(
    mut node: Ptr<YGNode>,
    mut measureFunc: FnPtr<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
) {
    ({
        facebook_yoga_NodeImpl::setMeasureFunc(
            &({ resolveRef_181((node).clone()) }),
            (measureFunc).clone(),
        )
    });
}
pub fn YGNodeHasMeasureFunc_227(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::hasMeasureFunc(&({ resolveRef_182(node) })) });
}
pub fn YGNodeSetMinContentMeasureFunc_228(
    mut node: Ptr<YGNode>,
    mut minContentMeasureFunc: FnPtr<
        fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
    >,
) {
    ({
        facebook_yoga_NodeImpl::setMinContentMeasureFunc(
            &({ resolveRef_181((node).clone()) }),
            (minContentMeasureFunc).clone(),
        )
    });
}
pub fn YGNodeHasMinContentMeasureFunc_229(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::hasMinContentMeasureFunc(&({ resolveRef_182(node) })) });
}
pub fn YGNodeSetMinContentWidth_230(mut node: Ptr<YGNode>, mut minContentWidth: ()) {
    ({
        facebook_yoga_NodeImpl::setMinContentWidth(
            &({ resolveRef_181((node).clone()) }),
            facebook_yoga_FloatOptional::new_1({ minContentWidth }),
        )
    });
}
pub fn YGNodeSetMinContentHeight_231(mut node: Ptr<YGNode>, mut minContentHeight: ()) {
    ({
        facebook_yoga_NodeImpl::setMinContentHeight(
            &({ resolveRef_181((node).clone()) }),
            facebook_yoga_FloatOptional::new_1({ minContentHeight }),
        )
    });
}
pub fn YGNodeGetMinContentWidth_232(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_NodeImpl::getMinContentWidth(&({ resolveRef_182(node) })) }),
            ))
            .as_pointer(),
            YGUndefined_24.with(|rc| *rc.borrow()),
        )
    });
}
pub fn YGNodeGetMinContentHeight_233(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_NodeImpl::getMinContentHeight(&({ resolveRef_182(node) })) }),
            ))
            .as_pointer(),
            YGUndefined_24.with(|rc| *rc.borrow()),
        )
    });
}
pub fn YGNodeSetBaselineFunc_234(
    mut node: Ptr<YGNode>,
    mut baselineFunc: FnPtr<fn(Ptr<YGNode>, (), ()) -> ()>,
) {
    ({
        facebook_yoga_NodeImpl::setBaselineFunc(
            &({ resolveRef_181((node).clone()) }),
            (baselineFunc).clone(),
        )
    });
}
pub fn YGNodeHasBaselineFunc_235(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::hasBaselineFunc(&({ resolveRef_182(node) })) });
}
pub fn YGNodeSetIsReferenceBaseline_236(mut nodeRef: Ptr<YGNode>, mut isReferenceBaseline: ()) {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_181((nodeRef).clone()) });
    if ({ ({ facebook_yoga_NodeImpl::isReferenceBaseline(&node) }) } != { isReferenceBaseline }) {
        ({ facebook_yoga_NodeImpl::setIsReferenceBaseline(&node, isReferenceBaseline) });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&node) });
    }
}
pub fn YGNodeIsReferenceBaseline_237(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::isReferenceBaseline(&({ resolveRef_182(node) })) });
}
pub fn YGNodeSetNodeType_238(mut node: Ptr<YGNode>, mut nodeType: YGNodeType) {
    ({
        facebook_yoga_NodeImpl::setNodeType(
            &({ resolveRef_181(node) }),
            ({ scopedEnum_92(nodeType) }),
        )
    });
    return;
}
pub fn YGNodeGetNodeType_239(mut node: Ptr<YGNode>) -> YGNodeType {
    return ({
        unscopedEnum_93(({ facebook_yoga_NodeImpl::getNodeType(&({ resolveRef_182(node) })) }))
    });
}
pub fn YGNodeSetAlwaysFormsContainingBlock_240(
    mut node: Ptr<YGNode>,
    mut alwaysFormsContainingBlock: (),
) {
    ({
        facebook_yoga_NodeImpl::setAlwaysFormsContainingBlock(
            &({ resolveRef_181((node).clone()) }),
            alwaysFormsContainingBlock,
        )
    });
}
pub fn YGNodeGetAlwaysFormsContainingBlock_241(mut node: Ptr<YGNode>) -> () {
    return ({ facebook_yoga_NodeImpl::alwaysFormsContainingBlock(&({ resolveRef_182(node) })) });
}
pub fn YGNodeCanUseCachedMeasurement_242(
    mut widthMode: YGMeasureMode,
    mut availableWidth: (),
    mut heightMode: YGMeasureMode,
    mut availableHeight: (),
    mut lastWidthMode: YGMeasureMode,
    mut lastAvailableWidth: (),
    mut lastHeightMode: YGMeasureMode,
    mut lastAvailableHeight: (),
    mut lastComputedWidth: (),
    mut lastComputedHeight: (),
    mut marginRow: (),
    mut marginColumn: (),
    mut config: Ptr<YGConfig>,
) -> () {
    return ({
        let _widthMode: facebook_yoga_SizingMode =
            ({ sizingMode_98(({ scopedEnum_87(widthMode) })) });
        let _heightMode: facebook_yoga_SizingMode =
            ({ sizingMode_98(({ scopedEnum_87(heightMode) })) });
        let _lastWidthMode: facebook_yoga_SizingMode =
            ({ sizingMode_98(({ scopedEnum_87(lastWidthMode) })) });
        let _lastHeightMode: facebook_yoga_SizingMode =
            ({ sizingMode_98(({ scopedEnum_87(lastHeightMode) })) });
        canUseCachedMeasurement_243(
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
            ({ resolveRef_63(config) }),
        )
    });
}
pub fn getResolvedLayoutProperty_244(mut nodeRef: Ptr<YGNode>, mut edge: facebook_yoga_Edge) -> () {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_182((nodeRef).clone()) });
    ({
        assertFatalWithNode_214(
            (node).clone(),
            (edge <= facebook_yoga_Edge_End),
            Ptr::<()>::from_string_literal(
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
pub fn getResolvedLayoutProperty_245(mut nodeRef: Ptr<YGNode>, mut edge: facebook_yoga_Edge) -> () {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_182((nodeRef).clone()) });
    ({
        assertFatalWithNode_214(
            (node).clone(),
            (edge <= facebook_yoga_Edge_End),
            Ptr::<()>::from_string_literal(
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
pub fn getResolvedLayoutProperty_246(mut nodeRef: Ptr<YGNode>, mut edge: facebook_yoga_Edge) -> () {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_182((nodeRef).clone()) });
    ({
        assertFatalWithNode_214(
            (node).clone(),
            (edge <= facebook_yoga_Edge_End),
            Ptr::<()>::from_string_literal(
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
pub fn YGNodeLayoutGetLeft_247(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_PhysicalEdge_Left,
        )
    });
}
pub fn YGNodeLayoutGetTop_248(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_PhysicalEdge_Top,
        )
    });
}
pub fn YGNodeLayoutGetRight_249(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_PhysicalEdge_Right,
        )
    });
}
pub fn YGNodeLayoutGetBottom_250(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_PhysicalEdge_Bottom,
        )
    });
}
pub fn YGNodeLayoutGetWidth_251(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub fn YGNodeLayoutGetHeight_252(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub fn YGNodeLayoutGetDirection_253(mut node: Ptr<YGNode>) -> YGDirection {
    return ({
        unscopedEnum_78(
            ({
                facebook_yoga_LayoutResultsImpl::direction(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeLayoutGetHadOverflow_254(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::hadOverflow(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
        )
    });
}
pub fn YGNodeLayoutGetMargin_255(mut node: Ptr<YGNode>, mut edge: YGEdge) -> () {
    return ({ getResolvedLayoutProperty_244(node, ({ scopedEnum_82(edge) })) });
}
pub fn YGNodeLayoutGetBorder_256(mut node: Ptr<YGNode>, mut edge: YGEdge) -> () {
    return ({ getResolvedLayoutProperty_245(node, ({ scopedEnum_82(edge) })) });
}
pub fn YGNodeLayoutGetPadding_257(mut node: Ptr<YGNode>, mut edge: YGEdge) -> () {
    return ({ getResolvedLayoutProperty_246(node, ({ scopedEnum_82(edge) })) });
}
pub fn YGNodeLayoutGetRawHeight_258(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::rawDimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_Dimension_Height,
        )
    });
}
pub fn YGNodeLayoutGetRawWidth_259(mut node: Ptr<YGNode>) -> () {
    return ({
        facebook_yoga_LayoutResultsImpl::rawDimension(
            &({ facebook_yoga_NodeImpl::getLayout_27(&({ resolveRef_182(node) })) }),
            facebook_yoga_Dimension_Width,
        )
    });
}
pub fn updateStyle_260(mut node: Ptr<YGNode>, mut value: facebook_yoga_Direction) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::direction(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Direction = value;
            facebook_yoga_StyleImpl::setDirection(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_261(mut node: Ptr<YGNode>, mut value: facebook_yoga_FlexDirection) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::flexDirection(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_FlexDirection = value;
            facebook_yoga_StyleImpl::setFlexDirection(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_262(mut node: Ptr<YGNode>, mut value: facebook_yoga_Justify) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::justifyContent(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_StyleImpl::setJustifyContent(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_263(mut node: Ptr<YGNode>, mut value: facebook_yoga_Justify) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::justifyItems(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_StyleImpl::setJustifyItems(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_264(mut node: Ptr<YGNode>, mut value: facebook_yoga_Justify) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::justifySelf(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Justify = value;
            facebook_yoga_StyleImpl::setJustifySelf(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_265(mut node: Ptr<YGNode>, mut value: facebook_yoga_Align) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::alignContent(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Align = value;
            facebook_yoga_StyleImpl::setAlignContent(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_266(mut node: Ptr<YGNode>, mut value: facebook_yoga_Align) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::alignItems(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Align = value;
            facebook_yoga_StyleImpl::setAlignItems(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_267(mut node: Ptr<YGNode>, mut value: facebook_yoga_Align) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::alignSelf(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Align = value;
            facebook_yoga_StyleImpl::setAlignSelf(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_268(mut node: Ptr<YGNode>, mut value: facebook_yoga_PositionType) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::positionType(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_PositionType = value;
            facebook_yoga_StyleImpl::setPositionType(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_269(mut node: Ptr<YGNode>, mut value: facebook_yoga_Wrap) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::flexWrap(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Wrap = value;
            facebook_yoga_StyleImpl::setFlexWrap(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_270(mut node: Ptr<YGNode>, mut value: facebook_yoga_Overflow) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::overflow(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Overflow = value;
            facebook_yoga_StyleImpl::setOverflow(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_271(mut node: Ptr<YGNode>, mut value: facebook_yoga_Display) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::display(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_Display = value;
            facebook_yoga_StyleImpl::setDisplay(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_272(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::flex(&style) });
        operator_eq_108(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setFlex(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_273(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::flexGrow(&style) });
        operator_eq_108(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setFlexGrow(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_274(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::flexShrink(&style) });
        operator_eq_108(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setFlexShrink(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_275(mut node: Ptr<YGNode>, value: facebook_yoga_StyleSizeLength) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_276(mut node: Ptr<YGNode>, mut value: facebook_yoga_FloatOptional) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if !({
        let _lhs: facebook_yoga_FloatOptional = ({ facebook_yoga_StyleImpl::aspectRatio(&style) });
        operator_eq_108(_lhs, (value).clone())
    }) {
        ({
            let _value: facebook_yoga_FloatOptional = (value).clone();
            facebook_yoga_StyleImpl::setAspectRatio(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_277(mut node: Ptr<YGNode>, mut value: facebook_yoga_BoxSizing) {
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
    if ({ ({ facebook_yoga_StyleImpl::boxSizing(&style) }) } != { value }) {
        ({
            let _value: facebook_yoga_BoxSizing = value;
            facebook_yoga_StyleImpl::setBoxSizing(&style, _value)
        });
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_278(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_279(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_280(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_281(mut node: Ptr<YGNode>, value: facebook_yoga_GridLine) {
    let value: Value<facebook_yoga_GridLine> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_282(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_283(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_284(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_285(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Edge,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_286(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Gutter,
    value: facebook_yoga_StyleLength,
) {
    let value: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_287(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Dimension,
    value: facebook_yoga_StyleSizeLength,
) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_288(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Dimension,
    value: facebook_yoga_StyleSizeLength,
) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn updateStyle_289(
    mut node: Ptr<YGNode>,
    mut idx: facebook_yoga_Dimension,
    value: facebook_yoga_StyleSizeLength,
) {
    let value: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(value));
    let style: Ptr<facebook_yoga_Style> =
        ({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) });
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
        ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
    }
}
pub fn YGNodeCopyStyle_290(mut dstNode: Ptr<YGNode>, mut srcNode: Ptr<YGNode>) {
    let mut dst: Ptr<facebook_yoga_Node> = ({ resolveRef_181((dstNode).clone()) });
    let mut src: Ptr<facebook_yoga_Node> = ({ resolveRef_182((srcNode).clone()) });
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
pub fn YGNodeStyleSetDirection_291(mut node: Ptr<YGNode>, mut value: YGDirection) {
    ({ updateStyle_260((node).clone(), ({ scopedEnum_77(value) })) });
}
pub fn YGNodeStyleGetDirection_292(mut node: Ptr<YGNode>) -> YGDirection {
    return ({
        unscopedEnum_78(
            ({
                facebook_yoga_StyleImpl::direction(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetFlexDirection_16(mut node: Ptr<YGNode>, mut flexDirection: YGFlexDirection) {
    ({ updateStyle_261((node).clone(), ({ scopedEnum_119(flexDirection) })) });
}
pub fn YGNodeStyleGetFlexDirection_293(mut node: Ptr<YGNode>) -> YGFlexDirection {
    return ({
        unscopedEnum_120(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetJustifyContent_294(mut node: Ptr<YGNode>, mut justifyContent: YGJustify) {
    ({ updateStyle_262((node).clone(), ({ scopedEnum_148(justifyContent) })) });
}
pub fn YGNodeStyleGetJustifyContent_295(mut node: Ptr<YGNode>) -> YGJustify {
    return ({
        unscopedEnum_149(
            ({
                facebook_yoga_StyleImpl::justifyContent(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetJustifyItems_296(mut node: Ptr<YGNode>, mut justifyItems: YGJustify) {
    ({ updateStyle_263((node).clone(), ({ scopedEnum_148(justifyItems) })) });
}
pub fn YGNodeStyleGetJustifyItems_297(mut node: Ptr<YGNode>) -> YGJustify {
    return ({
        unscopedEnum_149(
            ({
                facebook_yoga_StyleImpl::justifyItems(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetJustifySelf_298(mut node: Ptr<YGNode>, mut justifySelf: YGJustify) {
    ({ updateStyle_264((node).clone(), ({ scopedEnum_148(justifySelf) })) });
}
pub fn YGNodeStyleGetJustifySelf_299(mut node: Ptr<YGNode>) -> YGJustify {
    return ({
        unscopedEnum_149(
            ({
                facebook_yoga_StyleImpl::justifySelf(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetAlignContent_300(mut node: Ptr<YGNode>, mut alignContent: YGAlign) {
    ({ updateStyle_265((node).clone(), ({ scopedEnum_133(alignContent) })) });
}
pub fn YGNodeStyleGetAlignContent_301(mut node: Ptr<YGNode>) -> YGAlign {
    return ({
        unscopedEnum_134(
            ({
                facebook_yoga_StyleImpl::alignContent(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetAlignItems_302(mut node: Ptr<YGNode>, mut alignItems: YGAlign) {
    ({ updateStyle_266((node).clone(), ({ scopedEnum_133(alignItems) })) });
}
pub fn YGNodeStyleGetAlignItems_303(mut node: Ptr<YGNode>) -> YGAlign {
    return ({
        unscopedEnum_134(
            ({
                facebook_yoga_StyleImpl::alignItems(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetAlignSelf_304(mut node: Ptr<YGNode>, mut alignSelf: YGAlign) {
    ({ updateStyle_267((node).clone(), ({ scopedEnum_133(alignSelf) })) });
}
pub fn YGNodeStyleGetAlignSelf_305(mut node: Ptr<YGNode>) -> YGAlign {
    return ({
        unscopedEnum_134(
            ({
                facebook_yoga_StyleImpl::alignSelf(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetPositionType_306(mut node: Ptr<YGNode>, mut positionType: YGPositionType) {
    ({ updateStyle_268((node).clone(), ({ scopedEnum_158(positionType) })) });
}
pub fn YGNodeStyleGetPositionType_307(mut node: Ptr<YGNode>) -> YGPositionType {
    return ({
        unscopedEnum_159(
            ({
                facebook_yoga_StyleImpl::positionType(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetFlexWrap_308(mut node: Ptr<YGNode>, mut flexWrap: YGWrap) {
    ({ updateStyle_269((node).clone(), ({ scopedEnum_168(flexWrap) })) });
}
pub fn YGNodeStyleGetFlexWrap_309(mut node: Ptr<YGNode>) -> YGWrap {
    return ({
        unscopedEnum_169(
            ({
                facebook_yoga_StyleImpl::flexWrap(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetOverflow_310(mut node: Ptr<YGNode>, mut overflow: YGOverflow) {
    ({ updateStyle_270((node).clone(), ({ scopedEnum_153(overflow) })) });
}
pub fn YGNodeStyleGetOverflow_311(mut node: Ptr<YGNode>) -> YGOverflow {
    return ({
        unscopedEnum_154(
            ({
                facebook_yoga_StyleImpl::overflow(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetDisplay_312(mut node: Ptr<YGNode>, mut display: YGDisplay) {
    let mut wasContents: () = (({
        facebook_yoga_StyleImpl::display(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
        )
    }) == facebook_yoga_Display_Contents);
    ({ updateStyle_271((node).clone(), ({ scopedEnum_66(display) })) });
    let mut owner: Ptr<facebook_yoga_Node> =
        ({ facebook_yoga_NodeImpl::getOwner(&({ resolveRef_181((node).clone()) })) });
    if (!((owner).is_null()))
        && (wasContents != ((display as ()) == (YGDisplay_YGDisplayContents as ())))
    {
        ({
            let _children: Ptr<()> = ({ facebook_yoga_NodeImpl::getChildren(&owner) });
            facebook_yoga_NodeImpl::setChildren(&owner, _children)
        });
    }
}
pub fn YGNodeStyleGetDisplay_313(mut node: Ptr<YGNode>) -> YGDisplay {
    return ({
        unscopedEnum_67(
            ({
                facebook_yoga_StyleImpl::display(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetFlex_11(mut node: Ptr<YGNode>, mut flex: ()) {
    ({ updateStyle_272((node).clone(), facebook_yoga_FloatOptional::new_1({ flex })) });
}
pub fn YGNodeStyleGetFlex_314(mut nodeRef: Ptr<YGNode>) -> () {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_182((nodeRef).clone()) });
    return if ({
        facebook_yoga_FloatOptionalImpl::isUndefined(
            &Rc::new(RefCell::new(
                ({ facebook_yoga_StyleImpl::flex(&({ facebook_yoga_NodeImpl::style_25(&node) })) }),
            ))
            .as_pointer(),
        )
    }) {
        YGUndefined_24.with(|rc| *rc.borrow())
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
pub fn YGNodeStyleSetFlexGrow_15(mut node: Ptr<YGNode>, mut flexGrow: ()) {
    ({
        updateStyle_273(
            (node).clone(),
            facebook_yoga_FloatOptional::new_1({ flexGrow }),
        )
    });
}
pub fn YGNodeStyleGetFlexGrow_315(mut nodeRef: Ptr<YGNode>) -> () {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_182((nodeRef).clone()) });
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
        DefaultFlexGrow_178.with(|rc| *rc.borrow())
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
pub fn YGNodeStyleSetFlexShrink_316(mut node: Ptr<YGNode>, mut flexShrink: ()) {
    ({
        updateStyle_274(
            (node).clone(),
            facebook_yoga_FloatOptional::new_1({ flexShrink }),
        )
    });
}
pub fn YGNodeStyleGetFlexShrink_317(mut nodeRef: Ptr<YGNode>) -> () {
    let mut node: Ptr<facebook_yoga_Node> = ({ resolveRef_182((nodeRef).clone()) });
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
            WebDefaultFlexShrink_180.with(|rc| *rc.borrow())
        } else {
            DefaultFlexShrink_179.with(|rc| *rc.borrow())
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
pub fn YGNodeStyleSetFlexBasis_318(mut node: Ptr<YGNode>, mut flexBasis: ()) {
    ({
        updateStyle_275(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::points(flexBasis) }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisPercent_319(mut node: Ptr<YGNode>, mut flexBasisPercent: ()) {
    ({
        updateStyle_275(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::percent(flexBasisPercent) }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisAuto_320(mut node: Ptr<YGNode>) {
    ({
        updateStyle_275(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisMaxContent_321(mut node: Ptr<YGNode>) {
    ({
        updateStyle_275(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisFitContent_322(mut node: Ptr<YGNode>) {
    ({
        updateStyle_275(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetFlexBasisStretch_323(mut node: Ptr<YGNode>) {
    ({
        updateStyle_275(
            (node).clone(),
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetFlexBasis_324(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::flexBasis(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetPosition_325(mut node: Ptr<YGNode>, mut edge: YGEdge, mut points: ()) {
    ({
        updateStyle_282(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetPositionPercent_326(mut node: Ptr<YGNode>, mut edge: YGEdge, mut percent: ()) {
    ({
        updateStyle_282(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetPositionAuto_327(mut node: Ptr<YGNode>, mut edge: YGEdge) {
    ({
        updateStyle_282(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleGetPosition_328(mut node: Ptr<YGNode>, mut edge: YGEdge) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::position(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        ({ scopedEnum_82(edge) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMargin_329(mut node: Ptr<YGNode>, mut edge: YGEdge, mut points: ()) {
    ({
        updateStyle_283(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetMarginPercent_330(mut node: Ptr<YGNode>, mut edge: YGEdge, mut percent: ()) {
    ({
        updateStyle_283(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetMarginAuto_331(mut node: Ptr<YGNode>, mut edge: YGEdge) {
    ({
        updateStyle_283(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleGetMargin_332(mut node: Ptr<YGNode>, mut edge: YGEdge) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::margin(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        ({ scopedEnum_82(edge) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetPadding_333(mut node: Ptr<YGNode>, mut edge: YGEdge, mut points: ()) {
    ({
        updateStyle_284(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetPaddingPercent_334(mut node: Ptr<YGNode>, mut edge: YGEdge, mut percent: ()) {
    ({
        updateStyle_284(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleGetPadding_335(mut node: Ptr<YGNode>, mut edge: YGEdge) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::padding(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        ({ scopedEnum_82(edge) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetBorder_336(mut node: Ptr<YGNode>, mut edge: YGEdge, mut border: ()) {
    ({
        updateStyle_285(
            (node).clone(),
            ({ scopedEnum_82(edge) }),
            ({ facebook_yoga_StyleLength::points(border) }),
        )
    });
}
pub fn YGNodeStyleGetBorder_337(mut node: Ptr<YGNode>, mut edge: YGEdge) -> () {
    let border: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
        ({
            facebook_yoga_StyleImpl::border(
                &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182((node).clone()) })) }),
                ({ scopedEnum_82(edge) }),
            )
        }),
    ));
    if ({ facebook_yoga_StyleLengthImpl::isUndefined(&border.as_pointer()) })
        || ({ facebook_yoga_StyleLengthImpl::isAuto(&border.as_pointer()) })
    {
        return YGUndefined_24.with(|rc| *rc.borrow());
    }
    return { ({ facebook_yoga_StyleLengthImpl::to_YGValue(&border.as_pointer()) }).value };
}
pub fn YGNodeStyleSetGap_338(mut node: Ptr<YGNode>, mut gutter: YGGutter, mut gapLength: ()) {
    ({
        updateStyle_286(
            (node).clone(),
            ({ scopedEnum_143(gutter) }),
            ({ facebook_yoga_StyleLength::points(gapLength) }),
        )
    });
}
pub fn YGNodeStyleSetGapPercent_339(mut node: Ptr<YGNode>, mut gutter: YGGutter, mut percent: ()) {
    ({
        updateStyle_286(
            (node).clone(),
            ({ scopedEnum_143(gutter) }),
            ({ facebook_yoga_StyleLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleGetGap_340(mut node: Ptr<YGNode>, mut gutter: YGGutter) -> YGValue {
    return ({
        facebook_yoga_StyleLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::gap(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        ({ scopedEnum_143(gutter) }),
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetAspectRatio_341(mut node: Ptr<YGNode>, mut aspectRatio: ()) {
    ({
        updateStyle_276(
            (node).clone(),
            facebook_yoga_FloatOptional::new_1({ aspectRatio }),
        )
    });
}
pub fn YGNodeStyleGetAspectRatio_342(mut node: Ptr<YGNode>) -> () {
    let op: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            facebook_yoga_StyleImpl::aspectRatio(
                &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182((node).clone()) })) }),
            )
        }),
    ));
    return if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&op.as_pointer()) }) {
        YGUndefined_24.with(|rc| *rc.borrow())
    } else {
        ({ facebook_yoga_FloatOptionalImpl::unwrap(&op.as_pointer()) })
    };
}
pub fn YGNodeStyleSetBoxSizing_343(mut node: Ptr<YGNode>, mut boxSizing: YGBoxSizing) {
    ({ updateStyle_277((node).clone(), ({ scopedEnum_138(boxSizing) })) });
}
pub fn YGNodeStyleGetBoxSizing_344(mut node: Ptr<YGNode>) -> YGBoxSizing {
    return ({
        unscopedEnum_139(
            ({
                facebook_yoga_StyleImpl::boxSizing(
                    &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                )
            }),
        )
    });
}
pub fn YGNodeStyleSetWidth_8(mut node: Ptr<YGNode>, mut points: ()) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetWidthPercent_345(mut node: Ptr<YGNode>, mut percent: ()) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetWidthAuto_346(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleSetWidthMaxContent_347(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetWidthFitContent_348(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetWidthStretch_349(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetWidth_350(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::dimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        facebook_yoga_Dimension_Width,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetHeight_9(mut node: Ptr<YGNode>, mut points: ()) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::points(points) }),
        )
    });
}
pub fn YGNodeStyleSetHeightPercent_351(mut node: Ptr<YGNode>, mut percent: ()) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::percent(percent) }),
        )
    });
}
pub fn YGNodeStyleSetHeightAuto_352(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofAuto() }),
        )
    });
}
pub fn YGNodeStyleSetHeightMaxContent_353(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetHeightFitContent_354(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetHeightStretch_355(mut node: Ptr<YGNode>) {
    ({
        updateStyle_287(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetHeight_356(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::dimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        facebook_yoga_Dimension_Height,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMinWidth_357(mut node: Ptr<YGNode>, mut minWidth: ()) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::points(minWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthPercent_358(mut node: Ptr<YGNode>, mut minWidth: ()) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::percent(minWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthMaxContent_359(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthFitContent_360(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinWidthStretch_361(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMinWidth_362(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::minDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        facebook_yoga_Dimension_Width,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMinHeight_363(mut node: Ptr<YGNode>, mut minHeight: ()) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::points(minHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightPercent_364(mut node: Ptr<YGNode>, mut minHeight: ()) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::percent(minHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightMaxContent_365(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightFitContent_366(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMinHeightStretch_367(mut node: Ptr<YGNode>) {
    ({
        updateStyle_288(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMinHeight_368(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::minDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        facebook_yoga_Dimension_Height,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMaxWidth_369(mut node: Ptr<YGNode>, mut maxWidth: ()) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::points(maxWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthPercent_370(mut node: Ptr<YGNode>, mut maxWidth: ()) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::percent(maxWidth) }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthMaxContent_371(mut node: Ptr<YGNode>) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthFitContent_372(mut node: Ptr<YGNode>) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxWidthStretch_373(mut node: Ptr<YGNode>) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Width,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMaxWidth_374(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::maxDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        facebook_yoga_Dimension_Width,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetMaxHeight_375(mut node: Ptr<YGNode>, mut maxHeight: ()) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::points(maxHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightPercent_376(mut node: Ptr<YGNode>, mut maxHeight: ()) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::percent(maxHeight) }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightMaxContent_377(mut node: Ptr<YGNode>) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofMaxContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightFitContent_378(mut node: Ptr<YGNode>) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofFitContent() }),
        )
    });
}
pub fn YGNodeStyleSetMaxHeightStretch_379(mut node: Ptr<YGNode>) {
    ({
        updateStyle_289(
            (node).clone(),
            facebook_yoga_Dimension_Height,
            ({ facebook_yoga_StyleSizeLength::ofStretch() }),
        )
    });
}
pub fn YGNodeStyleGetMaxHeight_380(mut node: Ptr<YGNode>) -> YGValue {
    return ({
        facebook_yoga_StyleSizeLengthImpl::to_YGValue(
            &Rc::new(RefCell::new(
                ({
                    facebook_yoga_StyleImpl::maxDimension(
                        &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182(node) })) }),
                        facebook_yoga_Dimension_Height,
                    )
                }),
            ))
            .as_pointer(),
        )
    });
}
pub fn YGNodeStyleSetGridColumnStart_381(mut node: Ptr<YGNode>, mut gridColumnStart: ()) {
    ({
        updateStyle_278(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridColumnStart) }),
        )
    });
}
pub fn YGNodeStyleSetGridColumnStartAuto_382(mut node: Ptr<YGNode>) {
    ({ updateStyle_278((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridColumnStartSpan_383(mut node: Ptr<YGNode>, mut span: ()) {
    ({ updateStyle_278((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridColumnStart_384(mut node: Ptr<YGNode>) -> () {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridColumnStart(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0_()
    };
}
pub fn YGNodeStyleSetGridColumnEnd_385(mut node: Ptr<YGNode>, mut gridColumnEnd: ()) {
    ({
        updateStyle_279(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridColumnEnd) }),
        )
    });
}
pub fn YGNodeStyleSetGridColumnEndAuto_386(mut node: Ptr<YGNode>) {
    ({ updateStyle_279((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridColumnEndSpan_387(mut node: Ptr<YGNode>, mut span: ()) {
    ({ updateStyle_279((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridColumnEnd_388(mut node: Ptr<YGNode>) -> () {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridColumnEnd(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0_()
    };
}
pub fn YGNodeStyleSetGridRowStart_389(mut node: Ptr<YGNode>, mut gridRowStart: ()) {
    ({
        updateStyle_280(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridRowStart) }),
        )
    });
}
pub fn YGNodeStyleSetGridRowStartAuto_390(mut node: Ptr<YGNode>) {
    ({ updateStyle_280((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridRowStartSpan_391(mut node: Ptr<YGNode>, mut span: ()) {
    ({ updateStyle_280((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridRowStart_392(mut node: Ptr<YGNode>) -> () {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridRowStart(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0_()
    };
}
pub fn YGNodeStyleSetGridRowEnd_393(mut node: Ptr<YGNode>, mut gridRowEnd: ()) {
    ({
        updateStyle_281(
            (node).clone(),
            ({ facebook_yoga_GridLine::fromInteger(gridRowEnd) }),
        )
    });
}
pub fn YGNodeStyleSetGridRowEndAuto_394(mut node: Ptr<YGNode>) {
    ({ updateStyle_281((node).clone(), ({ facebook_yoga_GridLine::auto_() })) });
}
pub fn YGNodeStyleSetGridRowEndSpan_395(mut node: Ptr<YGNode>, mut span: ()) {
    ({ updateStyle_281((node).clone(), ({ facebook_yoga_GridLine::span(span) })) });
}
pub fn YGNodeStyleGetGridRowEnd_396(mut node: Ptr<YGNode>) -> () {
    let gridLine: Ptr<facebook_yoga_GridLine> = ({
        facebook_yoga_StyleImpl::gridRowEnd(
            &({ facebook_yoga_NodeImpl::style_25(&({ resolveRef_182((node).clone()) })) }),
        )
    });
    return if ({ facebook_yoga_GridLineImpl::isInteger(&gridLine) }) {
        gridLine.with(|__s| __s.integer)
    } else {
        0_()
    };
}
pub fn gridTrackSizeFromTypeAndValue_397(
    mut type_: YGGridTrackType,
    mut value: (),
) -> facebook_yoga_GridTrackSize {
    'switch: {
        match { (type_ as ()) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as ()) => {
                return ({ facebook_yoga_GridTrackSize::length(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as ()) => {
                return ({ facebook_yoga_GridTrackSize::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as ()) => {
                return ({ facebook_yoga_GridTrackSize::fr(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as ()) => {
                return ({ facebook_yoga_GridTrackSize::auto_() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as ()) => {
                return ({ facebook_yoga_GridTrackSize::auto_() });
            }
            _ => {}
        }
    };
    ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Unknown YGGridTrackType")) });
    panic!("ub: non-void function does not return a value")
}
pub fn styleSizeLengthFromTypeAndValue_398(
    mut type_: YGGridTrackType,
    mut value: (),
) -> facebook_yoga_StyleSizeLength {
    'switch: {
        match { (type_ as ()) } {
            __v if __v == (YGGridTrackType_YGGridTrackTypePoints as ()) => {
                return ({ facebook_yoga_StyleSizeLength::points(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypePercent as ()) => {
                return ({ facebook_yoga_StyleSizeLength::percent(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeFr as ()) => {
                return ({ facebook_yoga_StyleSizeLength::stretch(value) });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeAuto as ()) => {
                return ({ facebook_yoga_StyleSizeLength::ofAuto() });
            }
            __v if __v == (YGGridTrackType_YGGridTrackTypeMinmax as ()) => {
                return ({ facebook_yoga_StyleSizeLength::ofAuto() });
            }
            _ => {}
        }
    };
    ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Unknown YGGridTrackType")) });
    panic!("ub: non-void function does not return a value")
}
pub fn YGNodeStyleSetGridTemplateColumnsCount_399(mut node: Ptr<YGNode>, mut count: ()) {
    ({
        facebook_yoga_StyleImpl::resizeGridTemplateColumns(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateColumn_400(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateColumnMinMax_401(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateRowsCount_402(mut node: Ptr<YGNode>, mut count: ()) {
    ({
        facebook_yoga_StyleImpl::resizeGridTemplateRows(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateRow_403(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridTemplateRowMinMax_404(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridTemplateRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoColumnsCount_405(mut node: Ptr<YGNode>, mut count: ()) {
    ({
        facebook_yoga_StyleImpl::resizeGridAutoColumns(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoColumn_406(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoColumnMinMax_407(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoColumnAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoRowsCount_408(mut node: Ptr<YGNode>, mut count: ()) {
    ({
        facebook_yoga_StyleImpl::resizeGridAutoRows(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            count,
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoRow_409(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut type_: YGGridTrackType,
    mut value: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({ gridTrackSizeFromTypeAndValue_397(type_, value) }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGNodeStyleSetGridAutoRowMinMax_410(
    mut node: Ptr<YGNode>,
    mut index: (),
    mut minType: YGGridTrackType,
    mut minValue: (),
    mut maxType: YGGridTrackType,
    mut maxValue: (),
) {
    ({
        facebook_yoga_StyleImpl::setGridAutoRowAt(
            &({ facebook_yoga_NodeImpl::style_24(&({ resolveRef_181((node).clone()) })) }),
            index,
            ({
                let _min: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(minType, minValue) });
                let _max: facebook_yoga_StyleSizeLength =
                    ({ styleSizeLengthFromTypeAndValue_398(maxType, maxValue) });
                facebook_yoga_GridTrackSize::minmax(_min, _max)
            }),
        )
    });
    ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(&({ resolveRef_181((node).clone()) })) });
}
pub fn YGRoundValueToPixelGrid_411(
    mut value: (),
    mut pointScaleFactor: (),
    mut forceCeil: (),
    mut forceFloor: (),
) -> () {
    return ({ roundValueToPixelGrid_412(value, pointScaleFactor, forceCeil, forceFloor) });
}
thread_local!(
    pub static YGValueZero_2: Value<YGValue> = Rc::new(RefCell::new(YGValue {
        value: 0_(),
        unit: YGUnit_YGUnitPoint,
    }));
);
thread_local!(
    pub static YGValueUndefined_1: Value<YGValue> = Rc::new(RefCell::new(YGValue {
        value: YGUndefined_24.with(|rc| *rc.borrow()),
        unit: YGUnit_YGUnitUndefined,
    }));
);
thread_local!(
    pub static YGValueAuto_0: Value<YGValue> = Rc::new(RefCell::new(YGValue {
        value: YGUndefined_24.with(|rc| *rc.borrow()),
        unit: YGUnit_YGUnitAuto,
    }));
);
pub fn YGFloatIsUndefined_413(mut value: ()) -> () {
    return ({ isUndefined_99(value) });
}
pub fn resolveChildAlignment_414(
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
            isColumn_124(
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
pub fn resolveChildJustification_415(
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
pub fn fallbackAlignment_416(mut align: facebook_yoga_Align) -> facebook_yoga_Align {
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
pub fn fallbackAlignment_417(mut align: facebook_yoga_Justify) -> facebook_yoga_Justify {
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
pub fn paddingAndBorderForAxis_418(
    mut node: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut widthSize: (),
) -> () {
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
pub fn boundAxisWithinMinAndMax_419(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut value: facebook_yoga_FloatOptional,
    mut axisSize: (),
    mut widthSize: (),
) -> facebook_yoga_FloatOptional {
    let mut min: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    let mut max: facebook_yoga_FloatOptional = <facebook_yoga_FloatOptional>::default();
    if ({ isColumn_124(axis) }) {
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
    } else if ({ isRow_123(axis) }) {
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
        operator_ge_113(_lhs, facebook_yoga_FloatOptional::new_1({ 0_() }))
    }) && ({
        let _lhs: facebook_yoga_FloatOptional = (value).clone();
        operator_gt_112(_lhs, (max).clone())
    }) {
        return max;
    }
    if ({
        let _lhs: facebook_yoga_FloatOptional = (min).clone();
        operator_ge_113(_lhs, facebook_yoga_FloatOptional::new_1({ 0_() }))
    }) && ({
        let _lhs: facebook_yoga_FloatOptional = (value).clone();
        operator_lt_107(_lhs, (min).clone())
    }) {
        return min;
    }
    return value;
}
pub fn boundAxis_420(
    mut node: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut value: (),
    mut axisSize: (),
    mut widthSize: (),
) -> () {
    return ({
        let _a: () = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        boundAxisWithinMinAndMax_419(
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
        let _b: () = ({ paddingAndBorderForAxis_418((node).clone(), axis, direction, widthSize) });
        maxOrDefined_104(_a, _b)
    });
}
pub fn getPositionOfOppositeEdge_421(
    mut position: (),
    mut axis: facebook_yoga_FlexDirection,
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut node: Ptr<facebook_yoga_Node>,
) -> () {
    return ({
        ({
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&containingNode) }),
                    ({ dimension_131(axis) }),
                )
            })
        } - {
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    ({ dimension_131(axis) }),
                )
            })
        })
    } - { position });
}
pub fn setChildTrailingPosition_422(
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
) {
    ({
        let _position: () = ({
            let _position: () = ({
                facebook_yoga_LayoutResultsImpl::position(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    ({ flexStartEdge_127(axis) }),
                )
            });
            let _axis: facebook_yoga_FlexDirection = axis;
            let _node: Ptr<facebook_yoga_Node> = (child).clone();
            getPositionOfOppositeEdge_421(_position, _axis, (node).clone(), _node)
        });
        let _edge: facebook_yoga_PhysicalEdge = ({ flexEndEdge_128(axis) });
        facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
    });
}
pub fn needsTrailingPosition_423(mut axis: facebook_yoga_FlexDirection) -> () {
    return (axis == facebook_yoga_FlexDirection_RowReverse)
        || (axis == facebook_yoga_FlexDirection_ColumnReverse);
}
pub fn setFlexStartLayoutPosition_424(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: (),
) {
    let mut position: () = ({
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
                ({ flexStartEdge_127(axis) }),
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
                ({ flexStartEdge_127(axis) }),
            )
        });
    }
    ({
        facebook_yoga_NodeImpl::setLayoutPosition(&child, position, ({ flexStartEdge_127(axis) }))
    });
}
pub fn setFlexEndLayoutPosition_425(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: (),
) {
    let mut flexEndPosition: () = ({
        ({
            facebook_yoga_LayoutResultsImpl::border(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_128(axis) }),
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
                ({ flexEndEdge_128(axis) }),
            )
        });
    }
    ({
        let _position: () = ({
            getPositionOfOppositeEdge_421(flexEndPosition, axis, (parent).clone(), (child).clone())
        });
        let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_127(axis) });
        facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
    });
}
pub fn setCenterLayoutPosition_426(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: (),
) {
    let mut parentContentBoxSize: () = ({
        ({
            ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                    ({ dimension_131(axis) }),
                )
            })
        } - {
            ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                    ({ flexStartEdge_127(axis) }),
                )
            })
        })
    } - {
        ({
            facebook_yoga_LayoutResultsImpl::border(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_128(axis) }),
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
                ({ flexStartEdge_127(axis) }),
            )
        });
        parentContentBoxSize -= ({
            facebook_yoga_LayoutResultsImpl::padding(
                &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                ({ flexEndEdge_128(axis) }),
            )
        });
    }
    let mut childOuterSize: () = ({
        ({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                ({ dimension_131(axis) }),
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
    let mut position: () = ({
        ({ ((parentContentBoxSize - childOuterSize) / 2_f32) } + {
            ({
                facebook_yoga_LayoutResultsImpl::border(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&parent) }),
                    ({ flexStartEdge_127(axis) }),
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
                ({ flexStartEdge_127(axis) }),
            )
        });
    }
    ({
        facebook_yoga_NodeImpl::setLayoutPosition(&child, position, ({ flexStartEdge_127(axis) }))
    });
}
pub fn justifyAbsoluteChild_427(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: (),
) {
    let mut justify: facebook_yoga_Justify = if (({
        facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&parent) }))
    }) == facebook_yoga_Display_Grid)
    {
        ({ resolveChildJustification_415((parent).clone(), (child).clone()) })
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
                    setFlexStartLayoutPosition_424(
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
                    setFlexEndLayoutPosition_425(
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
                    setCenterLayoutPosition_426(
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
pub fn alignAbsoluteChild_428(
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut containingBlockWidth: (),
) {
    let mut itemAlign: facebook_yoga_Align =
        ({ resolveChildAlignment_414((parent).clone(), (child).clone()) });
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
                    setFlexStartLayoutPosition_424(
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
                    setFlexEndLayoutPosition_425(
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
                    setCenterLayoutPosition_426(
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
pub fn positionAbsoluteChild_429(
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut parent: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut isMainAxis: (),
    mut containingBlockWidth: (),
    mut containingBlockHeight: (),
) {
    let mut isAxisRow: () = ({ isRow_123(axis) });
    let mut containingBlockSize: () = if isAxisRow {
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
        let mut positionRelativeToInlineStart: () = ({
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
        let mut positionRelativeToFlexStart: () =
            if (({ inlineStartEdge_129(axis, direction) }) != ({ flexStartEdge_127(axis) })) {
                ({
                    getPositionOfOppositeEdge_421(
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
                ({ flexStartEdge_127(axis) }),
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
        let mut positionRelativeToInlineStart: () = ({
            ({
                ({
                    ({
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_27(&containingNode) }),
                                ({ dimension_131(axis) }),
                            )
                        })
                    } - {
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                ({ dimension_131(axis) }),
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
        let mut positionRelativeToFlexStart: () =
            if (({ inlineStartEdge_129(axis, direction) }) != ({ flexStartEdge_127(axis) })) {
                ({
                    getPositionOfOppositeEdge_421(
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
                ({ flexStartEdge_127(axis) }),
            )
        });
    } else {
        if isMainAxis {
            ({
                justifyAbsoluteChild_427(
                    (parent).clone(),
                    (child).clone(),
                    direction,
                    axis,
                    containingBlockWidth,
                )
            });
        } else {
            ({
                alignAbsoluteChild_428(
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
pub fn layoutAbsoluteChild_430(
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut containingBlockWidth: (),
    mut containingBlockHeight: (),
    mut widthMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) {
    let mut mainAxis: facebook_yoga_FlexDirection =
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_25(&node) })) })
            == facebook_yoga_Display_Grid)
        {
            ({ resolveDirection_125(facebook_yoga_FlexDirection_Row, direction) })
        } else {
            ({
                resolveDirection_125(
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
            ({ resolveCrossDirection_126(mainAxis, direction) })
        };
    let mut isMainAxisRow: () = ({ isRow_123(mainAxis) });
    let mut childWidth: () = YGUndefined_24.with(|rc| *rc.borrow());
    let mut childHeight: () = YGUndefined_24.with(|rc| *rc.borrow());
    let mut childWidthSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut childHeightSizingMode: facebook_yoga_SizingMode = facebook_yoga_SizingMode_MaxContent;
    let mut marginRow: () = ({
        facebook_yoga_StyleImpl::computeMarginForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&child) }),
            facebook_yoga_FlexDirection_Row,
            containingBlockWidth,
        )
    });
    let mut marginColumn: () = ({
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
                            let _referenceLength: () = containingBlockWidth;
                            let _ownerWidth: () = containingBlockWidth;
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
                let _axisSize: () = containingBlockWidth;
                let _widthSize: () = containingBlockWidth;
                boundAxis_420(
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
                boundAxis_420(
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
    if ((({ isUndefined_99(childWidth) }) ^ ({ isUndefined_99(childHeight) })) != 0) {
        if ({
            facebook_yoga_FloatOptionalImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({ facebook_yoga_StyleImpl::aspectRatio(&childStyle) }),
                ))
                .as_pointer(),
            )
        }) {
            if ({ isUndefined_99(childWidth) }) {
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
            } else if ({ isUndefined_99(childHeight) }) {
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
    if ({ isUndefined_99(childWidth) }) || ({ isUndefined_99(childHeight) }) {
        childWidthSizingMode = if ({ isUndefined_99(childWidth) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        childHeightSizingMode = if ({ isUndefined_99(childHeight) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
        if ((((!(isMainAxisRow)) && ({ isUndefined_99(childWidth) }))
            && (widthMode != facebook_yoga_SizingMode_MaxContent))
            && ({ isDefined_101(containingBlockWidth) }))
            && (containingBlockWidth > 0_())
        {
            childWidth = containingBlockWidth;
            childWidthSizingMode = facebook_yoga_SizingMode_FitContent;
        }
        ({
            let _node: Ptr<facebook_yoga_Node> = (child).clone();
            let _availableWidth: () = childWidth;
            let _availableHeight: () = childHeight;
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: () = containingBlockWidth;
            let _ownerHeight: () = containingBlockHeight;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutInternal_431(
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
        let _availableWidth: () = childWidth;
        let _availableHeight: () = childHeight;
        let _ownerDirection: facebook_yoga_Direction = direction;
        let _ownerWidth: () = containingBlockWidth;
        let _ownerHeight: () = containingBlockHeight;
        let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
        let _depth: () = depth;
        let _generationCount: () = generationCount;
        calculateLayoutInternal_431(
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
        positionAbsoluteChild_429(
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
        positionAbsoluteChild_429(
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
pub fn layoutAbsoluteDescendants_432(
    mut containingNode: Ptr<facebook_yoga_Node>,
    mut currentNode: Ptr<facebook_yoga_Node>,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut currentNodeDirection: facebook_yoga_Direction,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut currentDepth: (),
    mut generationCount: (),
    mut currentNodeLeftOffsetFromContainingBlock: (),
    mut currentNodeTopOffsetFromContainingBlock: (),
    mut containingNodeAvailableInnerWidth: (),
    mut containingNodeAvailableInnerHeight: (),
) -> () {
    let mut hasNewLayout: () = false;
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
        operator_ne_70(_a, __end2.as_pointer())
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
            let mut absoluteErrata: () = ({
                facebook_yoga_NodeImpl::hasErrata(
                    &currentNode,
                    facebook_yoga_Errata_AbsolutePercentAgainstInnerSize,
                )
            });
            let mut containingBlockWidth: () = if absoluteErrata {
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
            let mut containingBlockHeight: () = if absoluteErrata {
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
                let _containingBlockWidth: () = containingBlockWidth;
                let _containingBlockHeight: () = containingBlockHeight;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _direction: facebook_yoga_Direction = currentNodeDirection;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _depth: () = currentDepth;
                let _generationCount: () = generationCount;
                layoutAbsoluteChild_430(
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
                resolveDirection_125(
                    ({
                        facebook_yoga_StyleImpl::flexDirection(
                            &({ facebook_yoga_NodeImpl::style_24(&currentNode) }),
                        )
                    }),
                    currentNodeDirection,
                )
            });
            let mut parentCrossAxis: facebook_yoga_FlexDirection =
                ({ resolveCrossDirection_126(parentMainAxis, currentNodeDirection) });
            if ({ needsTrailingPosition_423(parentMainAxis) }) {
                let mut mainInsetsDefined: () = if ({ isRow_123(parentMainAxis) }) {
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
                    setChildTrailingPosition_422(
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
            if ({ needsTrailingPosition_423(parentCrossAxis) }) {
                let mut crossInsetsDefined: () = if ({ isRow_123(parentCrossAxis) }) {
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
                    setChildTrailingPosition_422(
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
            let mut childLeftPosition: () = ({
                facebook_yoga_LayoutResultsImpl::position(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    facebook_yoga_PhysicalEdge_Left,
                )
            });
            let mut childTopPosition: () = ({
                facebook_yoga_LayoutResultsImpl::position(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    facebook_yoga_PhysicalEdge_Top,
                )
            });
            let mut childLeftOffsetFromParent: () = if ({
                facebook_yoga_StyleImpl::horizontalInsetsDefined(
                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                )
            }) {
                (childLeftPosition - currentNodeLeftOffsetFromContainingBlock)
            } else {
                childLeftPosition
            };
            let mut childTopOffsetFromParent: () = if ({
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
            let mut childLeftOffsetFromContainingBlock: () =
                ({ currentNodeLeftOffsetFromContainingBlock } + {
                    ({
                        facebook_yoga_LayoutResultsImpl::position(
                            &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                            facebook_yoga_PhysicalEdge_Left,
                        )
                    })
                });
            let mut childTopOffsetFromContainingBlock: () =
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
                let _currentDepth: () = (currentDepth).wrapping_add(1_());
                let _generationCount: () = generationCount;
                let _currentNodeLeftOffsetFromContainingBlock: () =
                    childLeftOffsetFromContainingBlock;
                let _currentNodeTopOffsetFromContainingBlock: () =
                    childTopOffsetFromContainingBlock;
                let _containingNodeAvailableInnerWidth: () = containingNodeAvailableInnerWidth;
                let _containingNodeAvailableInnerHeight: () = containingNodeAvailableInnerHeight;
                layoutAbsoluteDescendants_432(
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
            ({ cleanupContentsNodesRecursively_433((child).clone(), hasNewLayout) });
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
                ((7_()) as facebook_yoga_Event_Type),
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
                ((8_()) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
}
pub fn calculateBaseline_434(mut node: Ptr<facebook_yoga_Node>) -> () {
    if ({ facebook_yoga_NodeImpl::hasBaselineFunc(&node) }) {
        ({ facebook_yoga_Event::publish_3_7((node).clone(), None) });
        let mut baseline: () = ({
            let _width: () = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_Dimension_Width,
                )
            });
            let _height: () = ({
                facebook_yoga_LayoutResultsImpl::measuredDimension(
                    &({ facebook_yoga_NodeImpl::getLayout_27(&node) }),
                    facebook_yoga_Dimension_Height,
                )
            });
            facebook_yoga_NodeImpl::baseline(&node, _width, _height)
        });
        ({ facebook_yoga_Event::publish_3_8((node).clone(), None) });
        ({
            assertFatalWithNode_214(
                (node).clone(),
                !(unimplemented!()),
                Ptr::<()>::from_string_literal(
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
        operator_ne_70(_a, __end2.as_pointer())
    }) {
        let mut child: Ptr<facebook_yoga_Node> = ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                &__begin2.as_pointer(),
            )
        });
        if (({ facebook_yoga_NodeImpl::getLineIndex(&child) }) > 0_()) {
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
        if (({ resolveChildAlignment_414((node).clone(), (child).clone()) })
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
    let mut baseline: () = ({ calculateBaseline_434((baselineChild).clone()) });
    return ({ baseline } + {
        ({
            facebook_yoga_LayoutResultsImpl::position(
                &({ facebook_yoga_NodeImpl::getLayout_26(&baselineChild) }),
                facebook_yoga_PhysicalEdge_Top,
            )
        })
    });
}
pub fn isBaselineLayout_435(mut node: Ptr<facebook_yoga_Node>) -> () {
    if ({
        isColumn_124(
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
        operator_ne_70(_a, __end2.as_pointer())
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
pub fn sizeIsExactAndMatchesOldMeasuredSize_436(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: (),
    mut lastComputedSize: (),
) -> () {
    return (sizeMode == facebook_yoga_SizingMode_StretchFit)
        && ({ inexactEquals_105(size, lastComputedSize) });
}
pub fn oldSizeIsMaxContentAndStillFits_437(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: (),
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastComputedSize: (),
) -> () {
    return ((sizeMode == facebook_yoga_SizingMode_FitContent)
        && (lastSizeMode == facebook_yoga_SizingMode_MaxContent))
        && ((size >= lastComputedSize) || ({ inexactEquals_105(size, lastComputedSize) }));
}
pub fn newSizeIsStricterAndStillValid_438(
    mut sizeMode: facebook_yoga_SizingMode,
    mut size: (),
    mut lastSizeMode: facebook_yoga_SizingMode,
    mut lastSize: (),
    mut lastComputedSize: (),
) -> () {
    return ((((((lastSizeMode == facebook_yoga_SizingMode_FitContent)
        && (sizeMode == facebook_yoga_SizingMode_FitContent))
        && ({ isDefined_101(lastSize) }))
        && ({ isDefined_101(size) }))
        && ({ isDefined_101(lastComputedSize) }))
        && (lastSize > size))
        && ((lastComputedSize <= size) || ({ inexactEquals_105(size, lastComputedSize) }));
}
pub fn canUseCachedMeasurement_243(
    mut widthMode: facebook_yoga_SizingMode,
    mut availableWidth: (),
    mut heightMode: facebook_yoga_SizingMode,
    mut availableHeight: (),
    mut lastWidthMode: facebook_yoga_SizingMode,
    mut lastAvailableWidth: (),
    mut lastHeightMode: facebook_yoga_SizingMode,
    mut lastAvailableHeight: (),
    mut lastComputedWidth: (),
    mut lastComputedHeight: (),
    mut marginRow: (),
    mut marginColumn: (),
    mut config: Ptr<facebook_yoga_Config>,
) -> () {
    if (({ isDefined_101(lastComputedHeight) }) && (lastComputedHeight < 0_()))
        || (({ isDefined_101(lastComputedWidth) }) && (lastComputedWidth < 0_()))
    {
        return false;
    }
    let mut pointScaleFactor: () = ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&config) });
    let mut useRoundedComparison: () = (!((config).is_null())) && (pointScaleFactor != 0_());
    let mut effectiveWidth: () = if useRoundedComparison {
        ({ roundValueToPixelGrid_412(availableWidth, pointScaleFactor, false, false) })
    } else {
        availableWidth
    };
    let mut effectiveHeight: () = if useRoundedComparison {
        ({ roundValueToPixelGrid_412(availableHeight, pointScaleFactor, false, false) })
    } else {
        availableHeight
    };
    let mut effectiveLastWidth: () = if useRoundedComparison {
        ({ roundValueToPixelGrid_412(lastAvailableWidth, pointScaleFactor, false, false) })
    } else {
        lastAvailableWidth
    };
    let mut effectiveLastHeight: () = if useRoundedComparison {
        ({ roundValueToPixelGrid_412(lastAvailableHeight, pointScaleFactor, false, false) })
    } else {
        lastAvailableHeight
    };
    let mut hasSameWidthSpec: () =
        (lastWidthMode == widthMode) && ({ inexactEquals_105(effectiveLastWidth, effectiveWidth) });
    let mut hasSameHeightSpec: () = (lastHeightMode == heightMode)
        && ({ inexactEquals_105(effectiveLastHeight, effectiveHeight) });
    let mut widthIsCompatible: () = (((hasSameWidthSpec)
        || ({
            sizeIsExactAndMatchesOldMeasuredSize_436(
                widthMode,
                (availableWidth - marginRow),
                lastComputedWidth,
            )
        }))
        || ({
            oldSizeIsMaxContentAndStillFits_437(
                widthMode,
                (availableWidth - marginRow),
                lastWidthMode,
                lastComputedWidth,
            )
        }))
        || ({
            newSizeIsStricterAndStillValid_438(
                widthMode,
                (availableWidth - marginRow),
                lastWidthMode,
                lastAvailableWidth,
                lastComputedWidth,
            )
        });
    let mut heightIsCompatible: () = (((hasSameHeightSpec)
        || ({
            sizeIsExactAndMatchesOldMeasuredSize_436(
                heightMode,
                (availableHeight - marginColumn),
                lastComputedHeight,
            )
        }))
        || ({
            oldSizeIsMaxContentAndStillFits_437(
                heightMode,
                (availableHeight - marginColumn),
                lastHeightMode,
                lastComputedHeight,
            )
        }))
        || ({
            newSizeIsStricterAndStillValid_438(
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
                ((5_()) as facebook_yoga_Event_Type),
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
                ((6_()) as facebook_yoga_Event_Type),
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
                ((2_()) as facebook_yoga_Event_Type),
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
                ((3_()) as facebook_yoga_Event_Type),
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
                ((4_()) as facebook_yoga_Event_Type),
                _arg2.as_pointer(),
            )
        });
    }
}
pub fn minOrDefined_439(mut a: (), mut b: ()) -> () {
    if ({ isDefined_101(a) }) && ({ isDefined_101(b) }) {
        return (unimplemented!().read());
    }
    return if ({ isUndefined_99(a) }) { b } else { a };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct facebook_yoga_FlexLineRunningLayout {
    #[offset(0)]
    pub totalFlexGrowFactors: (),
    #[offset(4)]
    pub totalFlexShrinkScaledFactors: (),
    #[offset(8)]
    pub remainingFreeSpace: (),
    #[offset(12)]
    pub mainDim: (),
    #[offset(16)]
    pub crossDim: (),
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
    pub itemsInFlow: (),
    #[offset(24)]
    pub sizeConsumed: (),
    #[offset(32)]
    pub numberOfAutoMargins: (),
    #[offset(40)]
    #[byte_size(20)]
    pub layout: facebook_yoga_FlexLineRunningLayout,
}
impl facebook_yoga_FlexLine {
    pub fn move_from(_a0: Ptr<facebook_yoga_FlexLine>) -> Self {
        Self {
            itemsInFlow: unimplemented!(),
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
                itemsInFlow: unimplemented!(),
                sizeConsumed: 0_f32,
                numberOfAutoMargins: 0_(),
                layout: <facebook_yoga_FlexLineRunningLayout>::default(),
            }
        }
    }
}
thread_local!(
    pub static gCurrentGenerationCount_440: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn hasAutoHorizontalMargin_441(style: Ptr<facebook_yoga_Style>) -> () {
    'loop_: for direction in IterRange::new(unimplemented!(), unimplemented!()) {
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
pub fn isColumnStretchEdge_442(
    mut owner: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
) -> () {
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
        && ({ isColumn_124(({ facebook_yoga_StyleImpl::flexDirection(&ownerStyle) })) }))
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
        && (!({ hasAutoHorizontalMargin_441((childStyle).clone()) })))
        && (({ resolveChildAlignment_414(owner, child) }) == facebook_yoga_Align_Stretch);
}
pub fn isInColumnStretchScrollSubtree_443(mut node: Ptr<facebook_yoga_Node>) -> () {
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
            || (!({ isColumnStretchEdge_442((owner).clone(), (current).clone()) }))
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
pub fn isNonZeroLength_444(length: Ptr<facebook_yoga_StyleLength>) -> () {
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
pub fn hasNonZeroVerticalSpacing_445(style: Ptr<facebook_yoga_Style>) -> () {
    let verticalEdges: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for edge in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut edge: facebook_yoga_Edge = (edge.read());
        if (({
            let _length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_StyleImpl::margin(&style, _edge)
                }),
            ));
            isNonZeroLength_444(_length.as_pointer())
        }) || ({
            let _length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_StyleImpl::padding(&style, _edge)
                }),
            ));
            isNonZeroLength_444(_length.as_pointer())
        })) || ({
            let _length: Value<facebook_yoga_StyleLength> = Rc::new(RefCell::new(
                ({
                    let _edge: facebook_yoga_Edge = edge;
                    facebook_yoga_StyleImpl::border(&style, _edge)
                }),
            ));
            isNonZeroLength_444(_length.as_pointer())
        }) {
            return true;
        }
    }
    return false;
}
pub fn hasPercentageLength_446(style: Ptr<facebook_yoga_Style>) -> () {
    let edges: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for edge in IterRange::new(unimplemented!(), unimplemented!()) {
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
    let dimensions: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: for dimension in IterRange::new(unimplemented!(), unimplemented!()) {
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
pub fn hasNonZeroFlex_447(node: Ptr<facebook_yoga_Node>) -> () {
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
    let mut canGrow: () =
        if ({ facebook_yoga_FloatOptionalImpl::isDefined(&flexGrow.as_pointer()) }) {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&flexGrow.as_pointer()) }) != 0_f32)
        } else {
            ({ facebook_yoga_FloatOptionalImpl::isDefined(&flex.as_pointer()) })
                && (({ facebook_yoga_FloatOptionalImpl::unwrap(&flex.as_pointer()) }) > 0_f32)
        };
    let mut canShrink: () =
        if ({ facebook_yoga_FloatOptionalImpl::isDefined(&flexShrink.as_pointer()) }) {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&flexShrink.as_pointer()) }) != 0_f32)
        } else {
            ((!((config).is_null())) && ({ facebook_yoga_ConfigImpl::useWebDefaults(&config) }))
                || (({ facebook_yoga_FloatOptionalImpl::isDefined(&flex.as_pointer()) })
                    && (({ facebook_yoga_FloatOptionalImpl::unwrap(&flex.as_pointer()) }) < 0_f32))
        };
    return (canGrow) || (canShrink);
}
pub fn isHeightFitContentIndependent_448(node: Ptr<facebook_yoga_Node>) -> () {
    let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_25(&node) });
    let height: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::dimension(&style, facebook_yoga_Dimension_Height) }),
    ));
    let flexBasis: Value<facebook_yoga_StyleSizeLength> = Rc::new(RefCell::new(
        ({ facebook_yoga_StyleImpl::flexBasis(&style) }),
    ));
    let mut hasRelativePercentPosition: () = ((({
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
    }))) && (!({ hasNonZeroFlex_447((node).clone()) })))
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
        && ({ isColumn_124(({ facebook_yoga_StyleImpl::flexDirection(&style) })) }))
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
        && (!({ hasNonZeroVerticalSpacing_445((style).clone()) })))
        && (!({ hasPercentageLength_446((style).clone()) }));
}
pub fn canSkipHeightFitContent_449(mut root: Ptr<facebook_yoga_Node>) -> () {
    if (root).is_null() {
        return false;
    }
    let mut maxPendingNodes: () = 64_();
    let stack: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut stackSize: () = 1_();
    'loop_: while (stackSize > 0_()) {
        let mut node: () = (unimplemented!().read());
        if ((node).is_null()) || (!({ isHeightFitContentIndependent_448((node).clone()) })) {
            return false;
        }
        let __tmp_0: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
            RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
        );
        let __range3: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> =
            __tmp_0.as_pointer();
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
            operator_ne_70(_a, __end3.as_pointer())
        }) {
            let mut child: Ptr<facebook_yoga_Node> = ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                    &__begin3.as_pointer(),
                )
            });
            if (stackSize == unimplemented!()) {
                return false;
            }
            unimplemented!().write((child).clone());
            ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_inc_4(
                    &__begin3.as_pointer(),
                )
            });
        }
    }
    return true;
}
pub fn constrainMaxSizeForMode_450(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut axis: facebook_yoga_FlexDirection,
    mut ownerAxisSize: (),
    mut ownerWidth: (),
    mut mode: Ptr<facebook_yoga_SizingMode>,
    mut size: Ptr<()>,
) {
    let maxSize: Value<facebook_yoga_FloatOptional> = Rc::new(RefCell::new(
        ({
            let _lhs: facebook_yoga_FloatOptional = ({
                facebook_yoga_StyleImpl::resolvedMaxDimension(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                    direction,
                    ({ dimension_131(axis) }),
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
            operator_add_111(_lhs, _rhs)
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
pub fn computeFlexBasisForChild_451(
    mut node: Ptr<facebook_yoga_Node>,
    mut child: Ptr<facebook_yoga_Node>,
    mut width: (),
    mut widthMode: facebook_yoga_SizingMode,
    mut height: (),
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut heightMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) {
    let mut mainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_125(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_25(&node) }),
                )
            }),
            direction,
        )
    });
    let mut isMainAxisRow: () = ({ isRow_123(mainAxis) });
    let mut mainAxisSize: () = if isMainAxisRow { width } else { height };
    let mut mainAxisOwnerSize: () = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let childWidth: Value<()> = Rc::new(RefCell::new(YGUndefined_24.with(|rc| *rc.borrow())));
    let childHeight: Value<()> = Rc::new(RefCell::new(YGUndefined_24.with(|rc| *rc.borrow())));
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
    let mut isRowStyleDimDefined: () = ({
        facebook_yoga_NodeImpl::hasDefiniteLength(&child, facebook_yoga_Dimension_Width, ownerWidth)
    });
    let mut isColumnStyleDimDefined: () = ({
        facebook_yoga_NodeImpl::hasDefiniteLength(
            &child,
            facebook_yoga_Dimension_Height,
            ownerHeight,
        )
    });
    let mut fixFlexBasisFitContent: () = ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    let mut useResolvedFlexBasis: () =
        ({ facebook_yoga_FloatOptionalImpl::isDefined(&resolvedFlexBasis.as_pointer()) })
            && ({ isDefined_101(mainAxisSize) });
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
                        paddingAndBorderForAxis_418(
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
                        maxOrDefined_115(
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
                    paddingAndBorderForAxis_418(
                        (child).clone(),
                        facebook_yoga_FlexDirection_Row,
                        direction,
                        ownerWidth,
                    )
                })
            });
        ({
            let _computedFlexBasis: facebook_yoga_FloatOptional = ({
                maxOrDefined_115(
                    ({
                        let _referenceLength: () = ownerWidth;
                        let _ownerWidth: () = ownerWidth;
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
                    paddingAndBorderForAxis_418(
                        (child).clone(),
                        facebook_yoga_FlexDirection_Column,
                        direction,
                        ownerWidth,
                    )
                })
            });
        ({
            let _computedFlexBasis: facebook_yoga_FloatOptional = ({
                maxOrDefined_115(
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
        let mut marginRow: () = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let mut marginColumn: () = ({
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
                                let _referenceLength: () = ownerWidth;
                                let _ownerWidth: () = ownerWidth;
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
            if ({ isUndefined_99((*childWidth.borrow())) }) && ({ isDefined_101(width) }) {
                (*childWidth.borrow_mut()) = width;
                (*childWidthSizingMode.borrow_mut()) = facebook_yoga_SizingMode_FitContent;
            }
        }
        let mut parentDoesNotScroll: () = (!((node).is_null()))
            && (({
                facebook_yoga_StyleImpl::overflow(&({ facebook_yoga_NodeImpl::style_25(&node) }))
            }) != facebook_yoga_Overflow_Scroll);
        let mut applyHeightFitContent: () = (isMainAxisRow) || (parentDoesNotScroll);
        if fixFlexBasisFitContent {
            let mut childHadOverflow: () = ((!((child).is_null()))
                && ({ facebook_yoga_NodeImpl::isDirty(&child) }))
                && ({
                    facebook_yoga_LayoutResultsImpl::hadOverflow(
                        &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                    )
                });
            let mut hasHeightIndependentSubtree: () =
                ((((((((!((node).is_null())) && (!((child).is_null()))) && (!(isMainAxisRow)))
                    && (parentDoesNotScroll))
                    && ({ isUndefined_99((*childHeight.borrow())) }))
                    && ({ isDefined_101(height) }))
                    && ({ isColumnStretchEdge_442((node).clone(), (child).clone()) }))
                    && ({ isInColumnStretchScrollSubtree_443((node).clone()) }))
                    && ({ canSkipHeightFitContent_449((child).clone()) });
            if (hasHeightIndependentSubtree) && (childHadOverflow) {
                ({ facebook_yoga_NodeImpl::setLayoutHadOverflow(&child, false) });
            }
            if hasHeightIndependentSubtree {
                applyHeightFitContent = false;
            }
        }
        if ((applyHeightFitContent) && ({ isUndefined_99((*childHeight.borrow())) }))
            && ({ isDefined_101(height) })
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
        let mut hasExactWidth: () =
            ({ isDefined_101(width) }) && (widthMode == facebook_yoga_SizingMode_StretchFit);
        let mut childWidthStretch: () =
            (({ resolveChildAlignment_414((node).clone(), (child).clone()) })
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
        let mut hasExactHeight: () =
            ({ isDefined_101(height) }) && (heightMode == facebook_yoga_SizingMode_StretchFit);
        let mut childHeightStretch: () =
            (({ resolveChildAlignment_414((node).clone(), (child).clone()) })
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
            let _ownerAxisSize: () = ownerWidth;
            let _ownerWidth: () = ownerWidth;
            constrainMaxSizeForMode_450(
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
            constrainMaxSizeForMode_450(
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
            let _availableWidth: () = (*childWidth.borrow());
            let _availableHeight: () = (*childHeight.borrow());
            let _ownerDirection: facebook_yoga_Direction = direction;
            let _widthSizingMode: facebook_yoga_SizingMode = (*childWidthSizingMode.borrow());
            let _heightSizingMode: facebook_yoga_SizingMode = (*childHeightSizingMode.borrow());
            let _ownerWidth: () = ownerWidth;
            let _ownerHeight: () = ownerHeight;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutInternal_431(
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
                        let _a: () = ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                ({ dimension_131(mainAxis) }),
                            )
                        });
                        let _b: () = ({
                            paddingAndBorderForAxis_418(
                                (child).clone(),
                                mainAxis,
                                direction,
                                ownerWidth,
                            )
                        });
                        maxOrDefined_104(_a, _b)
                    })
                });
            facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(&child, _computedFlexBasis)
        });
    }
    ({ facebook_yoga_NodeImpl::setLayoutComputedFlexBasisGeneration(&child, generationCount) });
}
pub fn measureNodeWithMeasureFunc_452(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut availableWidth: (),
    mut availableHeight: (),
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: (),
    mut ownerHeight: (),
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut reason: facebook_yoga_LayoutPassReason,
) {
    ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _condition: () = ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) });
        assertFatalWithNode_214(
            _node,
            _condition,
            Ptr::<()>::from_string_literal(b"Expected node to have custom measure function"),
        )
    });
    if (widthSizingMode == facebook_yoga_SizingMode_MaxContent) {
        availableWidth = YGUndefined_24.with(|rc| *rc.borrow());
    }
    if (heightSizingMode == facebook_yoga_SizingMode_MaxContent) {
        availableHeight = YGUndefined_24.with(|rc| *rc.borrow());
    }
    let layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    let mut paddingAndBorderAxisRow: () = ({
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
    let mut paddingAndBorderAxisColumn: () = ({
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
    let mut innerWidth: () = if ({ isUndefined_99(availableWidth) }) {
        availableWidth
    } else {
        ({ maxOrDefined_104(0_f32, (availableWidth - paddingAndBorderAxisRow)) })
    };
    let mut innerHeight: () = if ({ isUndefined_99(availableHeight) }) {
        availableHeight
    } else {
        ({ maxOrDefined_104(0_f32, (availableHeight - paddingAndBorderAxisColumn)) })
    };
    if (widthSizingMode == facebook_yoga_SizingMode_StretchFit)
        && (heightSizingMode == facebook_yoga_SizingMode_StretchFit)
    {
        ({
            let _measuredDimension: () = ({
                let _axisSize: () = ownerWidth;
                let _widthSize: () = ownerWidth;
                boundAxis_420(
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
            let _measuredDimension: () = ({
                boundAxis_420(
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
        ({ facebook_yoga_Event::publish_3_5((node).clone(), None) });
        let mut measuredSize: YGSize = ({
            let _widthMode: facebook_yoga_MeasureMode = ({ measureMode_96(widthSizingMode) });
            let _heightMode: facebook_yoga_MeasureMode = ({ measureMode_96(heightSizingMode) });
            facebook_yoga_NodeImpl::measure(&node, innerWidth, _widthMode, innerHeight, _heightMode)
        });
        {
            field!(layoutMarkerData, measureCallbacks).with_mut(|__v| *__v = *__v + 1_())
        };
        {
            unimplemented!().with_mut(|__v| *__v = *__v + 1_())
        };
        ({
            let _eventData: Value<
                facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_,
            > = Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackEnd_ {
                    width: innerWidth,
                    widthMeasureMode: ({ unscopedEnum_88(({ measureMode_96(widthSizingMode) })) }),
                    height: innerHeight,
                    heightMeasureMode: ({
                        unscopedEnum_88(({ measureMode_96(heightSizingMode) }))
                    }),
                    measuredWidth: measuredSize.width,
                    measuredHeight: measuredSize.height,
                    reason: reason,
                },
            ));
            facebook_yoga_Event::publish_3_6((node).clone(), _eventData.as_pointer())
        });
        ({
            let _measuredDimension: () = ({
                let _axisSize: () = ownerWidth;
                let _widthSize: () = ownerWidth;
                boundAxis_420(
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
            let _measuredDimension: () = ({
                boundAxis_420(
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
pub fn measureNodeWithoutChildren_453(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut availableWidth: (),
    mut availableHeight: (),
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: (),
    mut ownerHeight: (),
) {
    let layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    let mut width: () = availableWidth;
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
        let _measuredDimension: () = ({
            let _axisSize: () = ownerWidth;
            let _widthSize: () = ownerWidth;
            boundAxis_420(
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
    let mut height: () = availableHeight;
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
        let _measuredDimension: () = ({
            boundAxis_420(
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
pub fn isFixedSize_454(mut dim: (), mut sizingMode: facebook_yoga_SizingMode) -> () {
    return (sizingMode == facebook_yoga_SizingMode_StretchFit)
        || ((({ isDefined_101(dim) }) && (sizingMode == facebook_yoga_SizingMode_FitContent))
            && (dim <= 0_f64));
}
pub fn measureNodeWithFixedSize_455(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut availableWidth: (),
    mut availableHeight: (),
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: (),
    mut ownerHeight: (),
) -> () {
    if ({ isFixedSize_454(availableWidth, widthSizingMode) })
        && ({ isFixedSize_454(availableHeight, heightSizingMode) })
    {
        ({
            let _measuredDimension: () = ({
                let _axisSize: () = ownerWidth;
                let _widthSize: () = ownerWidth;
                boundAxis_420(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Row,
                    direction,
                    if ({ isUndefined_99(availableWidth) })
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
            let _measuredDimension: () = ({
                boundAxis_420(
                    (node).clone(),
                    facebook_yoga_FlexDirection_Column,
                    direction,
                    if ({ isUndefined_99(availableHeight) })
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
pub fn zeroOutLayoutRecursively_456(mut node: Ptr<facebook_yoga_Node>) {
    let __rhs = <facebook_yoga_LayoutResults>::default();
    ({ facebook_yoga_NodeImpl::getLayout_26(&node) }).write(__rhs);
    ({ facebook_yoga_NodeImpl::setLayoutDimension(&node, 0_(), facebook_yoga_Dimension_Width) });
    ({ facebook_yoga_NodeImpl::setLayoutDimension(&node, 0_(), facebook_yoga_Dimension_Height) });
    ({ facebook_yoga_NodeImpl::setHasNewLayout(&node, true) });
    ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&node) });
    let __range2: Ptr<()> = ({ facebook_yoga_NodeImpl::getChildren(&node) });
    'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
        ({ zeroOutLayoutRecursively_456((child).clone()) });
    }
}
pub fn cleanupContentsNodesRecursively_433(
    mut node: Ptr<facebook_yoga_Node>,
    mut didPerformLayout: (),
) {
    if ({ facebook_yoga_NodeImpl::hasContentsChildren(&node) }) {
        {
            ({ facebook_yoga_NodeImpl::cloneContentsChildrenIfNeeded(&node) });
            let __range3: Ptr<()> = ({ facebook_yoga_NodeImpl::getChildren(&node) });
            'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
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
                            0_(),
                            facebook_yoga_Dimension_Width,
                        )
                    });
                    ({
                        facebook_yoga_NodeImpl::setLayoutDimension(
                            &child,
                            0_(),
                            facebook_yoga_Dimension_Height,
                        )
                    });
                    if didPerformLayout {
                        ({ facebook_yoga_NodeImpl::setHasNewLayout(&child, true) });
                    }
                    ({ facebook_yoga_NodeImpl::setDirty(&child, false) });
                    ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&child) });
                    ({ cleanupContentsNodesRecursively_433((child).clone(), didPerformLayout) });
                }
            }
        };
    }
}
pub fn calculateAvailableInnerDimension_457(
    mut node: Ptr<facebook_yoga_Node>,
    mut direction: facebook_yoga_Direction,
    mut dimension: facebook_yoga_Dimension,
    mut availableDim: (),
    mut paddingAndBorder: (),
    mut ownerDim: (),
    mut ownerWidth: (),
) -> () {
    let mut availableInnerDim: () = (availableDim - paddingAndBorder);
    if ({ isDefined_101(availableInnerDim) }) {
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
        let mut minInnerDim: () = if ({
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
        let mut maxInnerDim: () = if ({
            facebook_yoga_FloatOptionalImpl::isUndefined(&maxDimensionOptional.as_pointer())
        }) {
            3.4028235e+38_f32
        } else {
            (({ facebook_yoga_FloatOptionalImpl::unwrap(&maxDimensionOptional.as_pointer()) })
                - paddingAndBorder)
        };
        let __rhs = ({
            maxOrDefined_104(
                ({ minOrDefined_439(availableInnerDim, maxInnerDim) }),
                minInnerDim,
            )
        });
        availableInnerDim = __rhs;
    }
    return availableInnerDim;
}
pub fn computeFlexBasisForChildren_458(
    mut node: Ptr<facebook_yoga_Node>,
    mut availableInnerWidth: (),
    mut availableInnerHeight: (),
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut performLayout: (),
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) -> () {
    let mut totalOuterFlexBasis: () = 0_f32;
    let mut singleFlexChild: Ptr<YGNode> = Ptr::<YGNode>::null();
    let children: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
        RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
    );
    let mut sizingModeMainDim: facebook_yoga_SizingMode = if ({ isRow_123(mainAxis) }) {
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
            operator_ne_70(_a, __end3.as_pointer())
        }) {
            let mut child: Ptr<facebook_yoga_Node> = ({
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_deref(
                    &__begin3.as_pointer(),
                )
            });
            if ({ facebook_yoga_NodeImpl::isNodeFlexible(&child) }) {
                if ((!((singleFlexChild).is_null()))
                    || ({
                        inexactEquals_105(
                            ({ facebook_yoga_NodeImpl::resolveFlexGrow(&child) }),
                            0_f32,
                        )
                    }))
                    || ({
                        inexactEquals_105(
                            ({ facebook_yoga_NodeImpl::resolveFlexShrink(&child) }),
                            0_f32,
                        )
                    })
                {
                    singleFlexChild = Ptr::<YGNode>::null();
                    break;
                } else {
                    singleFlexChild = (child).clone();
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
        operator_ne_70(_a, __end2.as_pointer())
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
                ({ zeroOutLayoutRecursively_456((child).clone()) });
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
        if ({ (child).clone() } == { (singleFlexChild).clone() }) {
            ({
                facebook_yoga_NodeImpl::setLayoutComputedFlexBasisGeneration(
                    &child,
                    generationCount,
                )
            });
            ({
                facebook_yoga_NodeImpl::setLayoutComputedFlexBasis(
                    &child,
                    facebook_yoga_FloatOptional::new_1({ 0_() }),
                )
            });
        } else {
            ({
                let _node: Ptr<facebook_yoga_Node> = (node).clone();
                let _child: Ptr<facebook_yoga_Node> = (child).clone();
                let _width: () = availableInnerWidth;
                let _widthMode: facebook_yoga_SizingMode = widthSizingMode;
                let _height: () = availableInnerHeight;
                let _ownerWidth: () = ownerWidth;
                let _ownerHeight: () = ownerHeight;
                let _heightMode: facebook_yoga_SizingMode = heightSizingMode;
                let _direction: facebook_yoga_Direction = direction;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _depth: () = depth;
                let _generationCount: () = generationCount;
                computeFlexBasisForChild_451(
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
pub fn computeMinContentMainSize_459(
    mut node: Ptr<facebook_yoga_Node>,
    mut requestedAxis: facebook_yoga_FlexDirection,
    mut ownerDirection: facebook_yoga_Direction,
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut generationCount: (),
) -> () {
    let mut wantRow: () = ({ isRow_123(requestedAxis) });
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
    let mut axisIndex: () = ({ to_underlying_38(({ dimension_131(requestedAxis) })) });
    if ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) }) {
        if ({ (unimplemented!().read()) } != { generationCount }) {
            let mut size: YGSize = if ({ facebook_yoga_NodeImpl::hasMinContentMeasureFunc(&node) })
            {
                ({
                    let _availableWidth: () = if wantRow {
                        0_f32
                    } else {
                        YGUndefined_24.with(|rc| *rc.borrow())
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: () = if wantRow {
                        YGUndefined_24.with(|rc| *rc.borrow())
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
                    let _availableWidth: () = if wantRow {
                        0_f32
                    } else {
                        YGUndefined_24.with(|rc| *rc.borrow())
                    };
                    let _widthMode: facebook_yoga_MeasureMode = if wantRow {
                        facebook_yoga_MeasureMode_AtMost
                    } else {
                        facebook_yoga_MeasureMode_Undefined
                    };
                    let _availableHeight: () = if wantRow {
                        YGUndefined_24.with(|rc| *rc.borrow())
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
            unimplemented!().write(if wantRow { size.width } else { size.height });
            unimplemented!().write(generationCount);
        }
        let mut leafDirection: facebook_yoga_Direction =
            ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
        let mut paddingAndBorder: () = ({
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
        return ({ (unimplemented!().read()) } + { paddingAndBorder });
    }
    if (({ facebook_yoga_NodeImpl::getChildCount(&node) }) == 0_()) {
        return 0_f32;
    }
    if ({ (unimplemented!().read()) } == { generationCount })
        && ({ inexactEquals_105((unimplemented!().read()), ownerWidth) })
    {
        return (unimplemented!().read());
    }
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    let mut nodeMainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_125(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            }),
            direction,
        )
    });
    let mut nodeCrossAxis: facebook_yoga_FlexDirection =
        ({ resolveCrossDirection_126(nodeMainAxis, direction) });
    let mut mainTotal: () = 0_f32;
    let mut crossMax: () = 0_f32;
    let mut i: () = 0_();
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
        let mut childMain: () = ({
            computeMinContentMainSize_459(
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
        let mut childCross: () = ({
            computeMinContentMainSize_459(
                (child).clone(),
                nodeCrossAxis,
                direction,
                ownerWidth,
                ownerHeight,
                generationCount,
            )
        });
        childCross += ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                nodeCrossAxis,
                ownerWidth,
            )
        });
        mainTotal += childMain;
        let __rhs = (unimplemented!().read());
        crossMax = __rhs;
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
    crossMax += ({
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
    let mut nodeMainIsRow: () = ({ isRow_123(nodeMainAxis) });
    let mut widthMin: () = if nodeMainIsRow { mainTotal } else { crossMax };
    let mut heightMin: () = if nodeMainIsRow { crossMax } else { mainTotal };
    let mut minContent: () = if wantRow { widthMin } else { heightMin };
    unimplemented!().write(minContent);
    unimplemented!().write(ownerWidth);
    unimplemented!().write(generationCount);
    return minContent;
}
pub fn computeAutoMinMainSize_460(
    mut child: Ptr<facebook_yoga_Node>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut ownerMainAxisSize: (),
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut generationCount: (),
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
                        ({ dimension_131(mainAxis) }),
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
    let mut mainDim: facebook_yoga_Dimension = ({ dimension_131(mainAxis) });
    let mut crossDim: facebook_yoga_Dimension = if ({ isRow_123(mainAxis) }) {
        facebook_yoga_Dimension_Height
    } else {
        facebook_yoga_Dimension_Width
    };
    let mut isMainAxisRow: () = ({ isRow_123(mainAxis) });
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
        let mut crossOwner: () = if isMainAxisRow {
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
            let mut ratio: () =
                ({ facebook_yoga_FloatOptionalImpl::unwrap(&aspectRatio.as_pointer()) });
            let mut crossValue: () =
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
            computeMinContentMainSize_459(
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
                operator_lt_107(_lhs, (*floor.borrow()).clone())
            })
        {
            (*floor.borrow_mut()) = (*specifiedMain.borrow()).clone();
        }
    } else if ({ facebook_yoga_FloatOptionalImpl::isDefined(&transferredMain.as_pointer()) }) {
        if ({ facebook_yoga_FloatOptionalImpl::isUndefined(&floor.as_pointer()) })
            || ({
                let _lhs: facebook_yoga_FloatOptional = (*transferredMain.borrow()).clone();
                operator_lt_107(_lhs, (*floor.borrow()).clone())
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
            operator_gt_112(_lhs, (*maxMain.borrow()).clone())
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
pub fn boundAxisWithAutoMin_461(
    mut child: Ptr<facebook_yoga_Node>,
    mut axis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut value: (),
    mut axisSize: (),
    mut widthSize: (),
) -> () {
    let mut bounded: () =
        ({ boundAxis_420((child).clone(), axis, direction, value, axisSize, widthSize) });
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
pub fn distributeFreeSpaceSecondPass_462(
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut node: Ptr<facebook_yoga_Node>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut ownerWidth: (),
    mut mainAxisOwnerSize: (),
    mut availableInnerMainDim: (),
    mut availableInnerCrossDim: (),
    mut availableInnerWidth: (),
    mut availableInnerHeight: (),
    mut mainAxisOverflows: (),
    mut sizingModeCrossDim: facebook_yoga_SizingMode,
    mut performLayout: (),
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) -> () {
    let mut childFlexBasis: () = 0_();
    let mut flexShrinkScaledFactor: () = 0_();
    let mut flexGrowFactor: () = 0_();
    let mut deltaFreeSpace: () = 0_();
    let mut isMainAxisRow: () = ({ isRow_123(mainAxis) });
    let mut isNodeFlexWrap: () =
        (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            != facebook_yoga_Wrap_NoWrap);
    'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut currentLineChild: Ptr<facebook_yoga_Node> = (unimplemented!().read());
        childFlexBasis = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _node: Ptr<facebook_yoga_Node> = (currentLineChild).clone();
                        let _value: facebook_yoga_FloatOptional =
                            ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild) })
                                .with(|__s| __s.computedFlexBasis.clone());
                        boundAxisWithinMinAndMax_419(
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
        let mut updatedMainSize: () = childFlexBasis;
        if ({ isDefined_101(flexLine.with(|__s| __s.layout.remainingFreeSpace)) })
            && (flexLine.with(|__s| __s.layout.remainingFreeSpace) < 0_())
        {
            flexShrinkScaledFactor = ({
                -({ facebook_yoga_NodeImpl::resolveFlexShrink(&currentLineChild) })
            } * { childFlexBasis });
            if (flexShrinkScaledFactor != 0_()) {
                let mut childSize: () = YGUndefined_24.with(|rc| *rc.borrow());
                let mut shrinkFactorMagnitude: () = unimplemented!();
                if ({ isDefined_101(flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors)) })
                    && ((shrinkFactorMagnitude < 1e-06_f32)
                        || (flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors) > 0_()))
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
                    boundAxisWithAutoMin_461(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        childSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
            }
        } else if ({ isDefined_101(flexLine.with(|__s| __s.layout.remainingFreeSpace)) })
            && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_())
        {
            flexGrowFactor = ({ facebook_yoga_NodeImpl::resolveFlexGrow(&currentLineChild) });
            if (!(unimplemented!())) && (flexGrowFactor != 0_()) {
                updatedMainSize = ({
                    boundAxisWithAutoMin_461(
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
        let mut marginMain: () = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                mainAxis,
                availableInnerWidth,
            )
        });
        let mut marginCross: () = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&currentLineChild) }),
                crossAxis,
                availableInnerWidth,
            )
        });
        let childCrossSize: Value<()> =
            Rc::new(RefCell::new(YGUndefined_24.with(|rc| *rc.borrow())));
        let childMainSize: Value<()> = Rc::new(RefCell::new((updatedMainSize + marginMain)));
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
        } else if ((((((!(unimplemented!()))
            && (!({
                facebook_yoga_NodeImpl::hasDefiniteLength(
                    &currentLineChild,
                    ({ dimension_131(crossAxis) }),
                    availableInnerCrossDim,
                )
            })))
            && (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit))
            && (!((isNodeFlexWrap) && (mainAxisOverflows))))
            && (({ resolveChildAlignment_414((node).clone(), (currentLineChild).clone()) })
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
                ({ dimension_131(crossAxis) }),
                availableInnerCrossDim,
            )
        }) {
            (*childCrossSize.borrow_mut()) = availableInnerCrossDim;
            (*childCrossSizingMode.borrow_mut()) =
                if ({ isUndefined_99((*childCrossSize.borrow())) }) {
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
                                    ({ dimension_131(crossAxis) }),
                                    availableInnerCrossDim,
                                    availableInnerWidth,
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                })
            } + { marginCross });
            let mut isLoosePercentageMeasurement: () = ({
                facebook_yoga_StyleSizeLengthImpl::isPercent(
                    &Rc::new(RefCell::new(
                        ({
                            facebook_yoga_NodeImpl::getProcessedDimension(
                                &currentLineChild,
                                ({ dimension_131(crossAxis) }),
                            )
                        }),
                    ))
                    .as_pointer(),
                )
            }) && (sizingModeCrossDim
                != facebook_yoga_SizingMode_StretchFit);
            (*childCrossSizingMode.borrow_mut()) =
                if ({ isUndefined_99((*childCrossSize.borrow())) })
                    || (isLoosePercentageMeasurement)
                {
                    facebook_yoga_SizingMode_MaxContent
                } else {
                    facebook_yoga_SizingMode_StretchFit
                };
        }
        ({
            constrainMaxSizeForMode_450(
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
            constrainMaxSizeForMode_450(
                (currentLineChild).clone(),
                direction,
                crossAxis,
                availableInnerCrossDim,
                availableInnerWidth,
                (childCrossSizingMode.as_pointer()),
                (childCrossSize.as_pointer()),
            )
        });
        let mut requiresStretchLayout: () = (((!({
            facebook_yoga_NodeImpl::hasDefiniteLength(
                &currentLineChild,
                ({ dimension_131(crossAxis) }),
                availableInnerCrossDim,
            )
        })) && (({
            resolveChildAlignment_414((node).clone(), (currentLineChild).clone())
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
        let mut childWidth: () = if isMainAxisRow {
            (*childMainSize.borrow())
        } else {
            (*childCrossSize.borrow())
        };
        let mut childHeight: () = if !(isMainAxisRow) {
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
        let mut isLayoutPass: () = (performLayout) && (!(requiresStretchLayout));
        ({
            let _node: Ptr<facebook_yoga_Node> = (currentLineChild).clone();
            let _availableWidth: () = childWidth;
            let _availableHeight: () = childHeight;
            let _ownerDirection: facebook_yoga_Direction = ({
                facebook_yoga_LayoutResultsImpl::direction(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                )
            });
            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
            let _ownerWidth: () = availableInnerWidth;
            let _ownerHeight: () = availableInnerHeight;
            let _performLayout: () = isLayoutPass;
            let _reason: facebook_yoga_LayoutPassReason = if isLayoutPass {
                facebook_yoga_LayoutPassReason_kFlexLayout
            } else {
                facebook_yoga_LayoutPassReason_kFlexMeasure
            };
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutInternal_431(
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
            let _hadOverflow: () = ({
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
pub fn distributeFreeSpaceFirstPass_463(
    mut node: Ptr<facebook_yoga_Node>,
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut direction: facebook_yoga_Direction,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut ownerWidth: (),
    mut mainAxisOwnerSize: (),
    mut availableInnerMainDim: (),
    mut availableInnerWidth: (),
) {
    let mut flexShrinkScaledFactor: () = 0_();
    let mut flexGrowFactor: () = 0_();
    let mut baseMainSize: () = 0_();
    let mut boundMainSize: () = 0_();
    let mut deltaFreeSpace: () = 0_();
    let mut useRunningTotals: () = ({
        facebook_yoga_NodeImpl::hasErrata(
            &node,
            facebook_yoga_Errata_FlexFirstPassUsesRunningTotals,
        )
    });
    let mut originalTotalFlexGrowFactors: () = flexLine.with(|__s| __s.layout.totalFlexGrowFactors);
    let mut originalTotalFlexShrinkScaledFactors: () =
        flexLine.with(|__s| __s.layout.totalFlexShrinkScaledFactors);
    'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut currentLineChild: Ptr<facebook_yoga_Node> = (unimplemented!().read());
        let mut childFlexBasis: () = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _node: Ptr<facebook_yoga_Node> = (currentLineChild).clone();
                        let _value: facebook_yoga_FloatOptional =
                            ({ facebook_yoga_NodeImpl::getLayout_26(&currentLineChild) })
                                .with(|__s| __s.computedFlexBasis.clone());
                        boundAxisWithinMinAndMax_419(
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
        if (flexLine.with(|__s| __s.layout.remainingFreeSpace) < 0_()) {
            flexShrinkScaledFactor = ({
                -({ facebook_yoga_NodeImpl::resolveFlexShrink(&currentLineChild) })
            } * { childFlexBasis });
            if ({ isDefined_101(flexShrinkScaledFactor) }) && (flexShrinkScaledFactor != 0_()) {
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
                    boundAxisWithAutoMin_461(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if (({ isDefined_101(baseMainSize) }) && ({ isDefined_101(boundMainSize) }))
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
        } else if ({ isDefined_101(flexLine.with(|__s| __s.layout.remainingFreeSpace)) })
            && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_())
        {
            flexGrowFactor = ({ facebook_yoga_NodeImpl::resolveFlexGrow(&currentLineChild) });
            if ({ isDefined_101(flexGrowFactor) }) && (flexGrowFactor != 0_()) {
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
                    boundAxis_420(
                        (currentLineChild).clone(),
                        mainAxis,
                        direction,
                        baseMainSize,
                        availableInnerMainDim,
                        availableInnerWidth,
                    )
                });
                if (({ isDefined_101(baseMainSize) }) && ({ isDefined_101(boundMainSize) }))
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
pub fn resolveFlexibleLength_464(
    mut node: Ptr<facebook_yoga_Node>,
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut ownerWidth: (),
    mut mainAxisOwnerSize: (),
    mut availableInnerMainDim: (),
    mut availableInnerCrossDim: (),
    mut availableInnerWidth: (),
    mut availableInnerHeight: (),
    mut mainAxisOverflows: (),
    mut sizingModeCrossDim: facebook_yoga_SizingMode,
    mut performLayout: (),
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) {
    let mut originalFreeSpace: () = flexLine.with(|__s| __s.layout.remainingFreeSpace);
    if !({
        facebook_yoga_NodeImpl::hasErrata(&node, facebook_yoga_Errata_MinSizeUndefinedInsteadOfAuto)
    }) {
        'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut currentLineChild: Ptr<facebook_yoga_Node> = (unimplemented!().read());
            let __rhs = ({
                computeAutoMinMainSize_460(
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
        'loop_: for currentLineChild in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut currentLineChild: Ptr<facebook_yoga_Node> = (unimplemented!().read());
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
        let _ownerWidth: () = ownerWidth;
        let _mainAxisOwnerSize: () = mainAxisOwnerSize;
        let _availableInnerMainDim: () = availableInnerMainDim;
        let _availableInnerWidth: () = availableInnerWidth;
        distributeFreeSpaceFirstPass_463(
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
    let mut distributedFreeSpace: () = ({
        let _flexLine: Ptr<facebook_yoga_FlexLine> = (flexLine).clone();
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _crossAxis: facebook_yoga_FlexDirection = crossAxis;
        let _direction: facebook_yoga_Direction = direction;
        let _ownerWidth: () = ownerWidth;
        let _mainAxisOwnerSize: () = mainAxisOwnerSize;
        let _availableInnerMainDim: () = availableInnerMainDim;
        let _availableInnerCrossDim: () = availableInnerCrossDim;
        let _availableInnerWidth: () = availableInnerWidth;
        let _availableInnerHeight: () = availableInnerHeight;
        let _mainAxisOverflows: () = mainAxisOverflows;
        let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
        let _performLayout: () = performLayout;
        let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
        let _depth: () = depth;
        let _generationCount: () = generationCount;
        distributeFreeSpaceSecondPass_462(
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
pub fn justifyMainAxis_465(
    mut node: Ptr<facebook_yoga_Node>,
    flexLine: Ptr<facebook_yoga_FlexLine>,
    mut mainAxis: facebook_yoga_FlexDirection,
    mut crossAxis: facebook_yoga_FlexDirection,
    mut direction: facebook_yoga_Direction,
    mut sizingModeMainDim: facebook_yoga_SizingMode,
    mut sizingModeCrossDim: facebook_yoga_SizingMode,
    mut mainAxisOwnerSize: (),
    mut ownerWidth: (),
    mut availableInnerMainDim: (),
    mut availableInnerCrossDim: (),
    mut availableInnerWidth: (),
    mut performLayout: (),
) {
    let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&node) });
    let mut leadingPaddingAndBorderMain: () = ({
        facebook_yoga_StyleImpl::computeFlexStartPaddingAndBorder(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let mut trailingPaddingAndBorderMain: () = ({
        facebook_yoga_StyleImpl::computeFlexEndPaddingAndBorder(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            direction,
            ownerWidth,
        )
    });
    let mut gap: () = ({
        facebook_yoga_StyleImpl::computeGapForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            mainAxis,
            availableInnerMainDim,
        )
    });
    if (sizingModeMainDim == facebook_yoga_SizingMode_FitContent)
        && (flexLine.with(|__s| __s.layout.remainingFreeSpace) > 0_())
    {
        if ({
            facebook_yoga_StyleSizeLengthImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _axis: facebook_yoga_Dimension = ({ dimension_131(mainAxis) });
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
                        let _axis: facebook_yoga_Dimension = ({ dimension_131(mainAxis) });
                        let _referenceLength: () = mainAxisOwnerSize;
                        let _ownerWidth: () = ownerWidth;
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
            let mut minAvailableMainDim: () = ({
                ({
                    ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({
                                    let _direction: facebook_yoga_Direction = direction;
                                    let _axis: facebook_yoga_Dimension =
                                        ({ dimension_131(mainAxis) });
                                    let _referenceLength: () = mainAxisOwnerSize;
                                    let _ownerWidth: () = ownerWidth;
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
            let mut occupiedSpaceByChildNodes: () = ({ availableInnerMainDim } - {
                flexLine.with(|__s| __s.layout.remainingFreeSpace)
            });
            field!(field!(flexLine, layout), remainingFreeSpace).write(
                ({ maxOrDefined_104(0_f32, (minAvailableMainDim - occupiedSpaceByChildNodes)) }),
            );
        } else {
            field!(field!(flexLine, layout), remainingFreeSpace).write(0_());
        }
    }
    let mut leadingMainDim: () = 0_();
    let mut betweenMainDim: () = gap;
    let mut justifyContent: facebook_yoga_Justify = if (flexLine
        .with(|__s| __s.layout.remainingFreeSpace)
        >= 0_())
    {
        ({
            facebook_yoga_StyleImpl::justifyContent(&({ facebook_yoga_NodeImpl::style_24(&node) }))
        })
    } else {
        ({
            fallbackAlignment_417(
                ({
                    facebook_yoga_StyleImpl::justifyContent(
                        &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    )
                }),
            )
        })
    };
    if (flexLine.with(|__s| __s.numberOfAutoMargins) == 0_()) {
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
                    leadingMainDim = (flexLine.with(|__s| __s.layout.remainingFreeSpace) / 2_());
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_FlexEnd => {
                    leadingMainDim = flexLine.with(|__s| __s.layout.remainingFreeSpace);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceBetween => {
                    if (unimplemented!() > 1_()) {
                        betweenMainDim += ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) }
                            / { (unimplemented!()).wrapping_sub(1_()) });
                    }
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceEvenly => {
                    leadingMainDim = ({ flexLine.with(|__s| __s.layout.remainingFreeSpace) } / {
                        (unimplemented!()).wrapping_add(1_())
                    });
                    betweenMainDim += leadingMainDim;
                    break 'switch;
                }
                __v if __v == facebook_yoga_Justify_SpaceAround => {
                    leadingMainDim = ({
                        (0.5_f32 * flexLine.with(|__s| __s.layout.remainingFreeSpace))
                    } / { unimplemented!() });
                    betweenMainDim += (leadingMainDim * 2_());
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
    field!(field!(flexLine, layout), crossDim).write(0_());
    let mut maxAscentForCurrentLine: () = 0_();
    let mut maxDescentForCurrentLine: () = 0_();
    let mut isNodeBaselineLayout: () = ({ isBaselineLayout_435((node).clone()) });
    'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
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
                        flexLine.with(|__s| __s.numberOfAutoMargins)
                    })
                };
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
        }
        if performLayout {
            ({
                let _position: () = ({
                    ({
                        let _physicalEdge: facebook_yoga_PhysicalEdge =
                            ({ flexStartEdge_127(mainAxis) });
                        facebook_yoga_LayoutResultsImpl::position(&childLayout, _physicalEdge)
                    })
                } + { flexLine.with(|__s| __s.layout.mainDim) });
                let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_127(mainAxis) });
                facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
            });
        }
        if ({ (child).clone() } != { (unimplemented!().read()) }) {
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
                        flexLine.with(|__s| __s.numberOfAutoMargins)
                    })
                };
                field!(field!(flexLine, layout), mainDim).with_mut(|__v| *__v = *__v + __rhs)
            };
        }
        let mut canSkipFlex: () =
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
                                    boundAxisWithinMinAndMax_419(
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
                let mut ascent: () = ({ ({ calculateBaseline_434((child).clone()) }) } + {
                    ({
                        facebook_yoga_StyleImpl::computeFlexStartMargin(
                            &({ facebook_yoga_NodeImpl::style_24(&child) }),
                            facebook_yoga_FlexDirection_Column,
                            direction,
                            availableInnerWidth,
                        )
                    })
                });
                let mut descent: () = ({
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
                let __rhs = ({ maxOrDefined_104(maxAscentForCurrentLine, ascent) });
                maxAscentForCurrentLine = __rhs;
                let __rhs = ({ maxOrDefined_104(maxDescentForCurrentLine, descent) });
                maxDescentForCurrentLine = __rhs;
            } else {
                let __rhs = ({
                    maxOrDefined_104(
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
pub fn calculateLayoutImpl_466(
    mut node: Ptr<facebook_yoga_Node>,
    mut availableWidth: (),
    mut availableHeight: (),
    mut ownerDirection: facebook_yoga_Direction,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut performLayout: (),
    mut reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) {
    ({
        assertFatalWithNode_214(
            (node).clone(),
            if ({ isUndefined_99(availableWidth) }) {
                (widthSizingMode == facebook_yoga_SizingMode_MaxContent)
            } else {
                true
            },
            Ptr::<()>::from_string_literal(
                b"availableWidth is indefinite so widthSizingMode must be SizingMode::MaxContent",
            ),
        )
    });
    ({
        assertFatalWithNode_214(
            (node).clone(),
            if ({ isUndefined_99(availableHeight) }) {
                (heightSizingMode == facebook_yoga_SizingMode_MaxContent)
            } else {
                true
            },
            Ptr::<()>::from_string_literal(
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
        .with_mut(|__v| *__v = *__v + 1_())
    };
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    ({ facebook_yoga_NodeImpl::setLayoutDirection(&node, direction) });
    let mut fixFlexBasisFitContent: () = ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    });
    if (fixFlexBasisFitContent) && (performLayout) {
        ({ facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, false) });
    }
    let mut flexRowDirection: facebook_yoga_FlexDirection =
        ({ resolveDirection_125(facebook_yoga_FlexDirection_Row, direction) });
    let mut flexColumnDirection: facebook_yoga_FlexDirection =
        ({ resolveDirection_125(facebook_yoga_FlexDirection_Column, direction) });
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
    let mut marginRowLeading: () = ({
        facebook_yoga_StyleImpl::computeInlineStartMargin(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    ({ facebook_yoga_NodeImpl::setLayoutMargin(&node, marginRowLeading, startEdge) });
    let mut marginRowTrailing: () = ({
        facebook_yoga_StyleImpl::computeInlineEndMargin(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            flexRowDirection,
            direction,
            ownerWidth,
        )
    });
    ({ facebook_yoga_NodeImpl::setLayoutMargin(&node, marginRowTrailing, endEdge) });
    let mut marginColumnLeading: () = ({
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
    let mut marginColumnTrailing: () = ({
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
    let mut marginAxisRow: () = (marginRowLeading + marginRowTrailing);
    let mut marginAxisColumn: () = (marginColumnLeading + marginColumnTrailing);
    ({
        let _border: () = ({
            facebook_yoga_StyleImpl::computeInlineStartBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, startEdge)
    });
    ({
        let _border: () = ({
            facebook_yoga_StyleImpl::computeInlineEndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexRowDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, endEdge)
    });
    ({
        let _border: () = ({
            facebook_yoga_StyleImpl::computeInlineStartBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, facebook_yoga_PhysicalEdge_Top)
    });
    ({
        let _border: () = ({
            facebook_yoga_StyleImpl::computeInlineEndBorder(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                flexColumnDirection,
                direction,
            )
        });
        facebook_yoga_NodeImpl::setLayoutBorder(&node, _border, facebook_yoga_PhysicalEdge_Bottom)
    });
    ({
        let _padding: () = ({
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
        let _padding: () = ({
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
        let _padding: () = ({
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
        let _padding: () = ({
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
            let _availableWidth: () = (availableWidth - marginAxisRow);
            let _availableHeight: () = (availableHeight - marginAxisColumn);
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: () = ownerWidth;
            let _ownerHeight: () = ownerHeight;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _reason: facebook_yoga_LayoutPassReason = reason;
            measureNodeWithMeasureFunc_452(
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
        ({ cleanupContentsNodesRecursively_433((node).clone(), performLayout) });
        return;
    }
    let mut childCount: () = ({ facebook_yoga_NodeImpl::getLayoutChildCount(&node) });
    if (childCount == 0_()) {
        ({
            measureNodeWithoutChildren_453(
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
        ({ cleanupContentsNodesRecursively_433((node).clone(), performLayout) });
        return;
    }
    if (!(performLayout))
        && ({
            measureNodeWithFixedSize_455(
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
        ({ cleanupContentsNodesRecursively_433((node).clone(), false) });
        return;
    }
    ({ facebook_yoga_NodeImpl::cloneChildrenIfNeeded(&node) });
    if (!(fixFlexBasisFitContent)) || (!(performLayout)) {
        ({ facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, false) });
    }
    ({ cleanupContentsNodesRecursively_433((node).clone(), performLayout) });
    let mut mainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_125(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            }),
            direction,
        )
    });
    let mut crossAxis: facebook_yoga_FlexDirection =
        ({ resolveCrossDirection_126(mainAxis, direction) });
    let mut isMainAxisRow: () = ({ isRow_123(mainAxis) });
    let mut isNodeFlexWrap: () =
        (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            != facebook_yoga_Wrap_NoWrap);
    let mut mainAxisOwnerSize: () = if isMainAxisRow {
        ownerWidth
    } else {
        ownerHeight
    };
    let mut crossAxisOwnerSize: () = if isMainAxisRow {
        ownerHeight
    } else {
        ownerWidth
    };
    let mut paddingAndBorderAxisMain: () =
        ({ paddingAndBorderForAxis_418((node).clone(), mainAxis, direction, ownerWidth) });
    let mut paddingAndBorderAxisCross: () =
        ({ paddingAndBorderForAxis_418((node).clone(), crossAxis, direction, ownerWidth) });
    let mut leadingPaddingAndBorderCross: () = ({
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
    let mut paddingAndBorderAxisRow: () = if isMainAxisRow {
        paddingAndBorderAxisMain
    } else {
        paddingAndBorderAxisCross
    };
    let mut paddingAndBorderAxisColumn: () = if isMainAxisRow {
        paddingAndBorderAxisCross
    } else {
        paddingAndBorderAxisMain
    };
    let mut availableInnerWidth: () = ({
        let _ownerDim: () = ownerWidth;
        let _ownerWidth: () = ownerWidth;
        calculateAvailableInnerDimension_457(
            (node).clone(),
            direction,
            facebook_yoga_Dimension_Width,
            (availableWidth - marginAxisRow),
            paddingAndBorderAxisRow,
            _ownerDim,
            _ownerWidth,
        )
    });
    let mut availableInnerHeight: () = ({
        calculateAvailableInnerDimension_457(
            (node).clone(),
            direction,
            facebook_yoga_Dimension_Height,
            (availableHeight - marginAxisColumn),
            paddingAndBorderAxisColumn,
            ownerHeight,
            ownerWidth,
        )
    });
    let mut availableInnerMainDim: () = if isMainAxisRow {
        availableInnerWidth
    } else {
        availableInnerHeight
    };
    let mut availableInnerCrossDim: () = if isMainAxisRow {
        availableInnerHeight
    } else {
        availableInnerWidth
    };
    let mut totalMainDim: () = 0_();
    totalMainDim += ({
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        let _availableInnerWidth: () = availableInnerWidth;
        let _availableInnerHeight: () = availableInnerHeight;
        let _ownerWidth: () = availableInnerWidth;
        let _ownerHeight: () = availableInnerHeight;
        let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
        let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
        let _direction: facebook_yoga_Direction = direction;
        let _mainAxis: facebook_yoga_FlexDirection = mainAxis;
        let _performLayout: () = performLayout;
        let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
        let _depth: () = depth;
        let _generationCount: () = generationCount;
        computeFlexBasisForChildren_458(
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
    if (childCount > 1_()) {
        totalMainDim += ({
            ({
                facebook_yoga_StyleImpl::computeGapForAxis(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                    mainAxis,
                    availableInnerMainDim,
                )
            })
        } * { (childCount).wrapping_sub(1_()) });
    }
    let mut mainAxisOverflows: () = (sizingModeMainDim != facebook_yoga_SizingMode_MaxContent)
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
    let mut lineCount: () = 0_();
    let mut totalLineCrossDim: () = 0_();
    let mut crossAxisGap: () = ({
        facebook_yoga_StyleImpl::computeGapForAxis(
            &({ facebook_yoga_NodeImpl::style_24(&node) }),
            crossAxis,
            availableInnerCrossDim,
        )
    });
    let mut maxLineMainDim: () = 0_();
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
        operator_ne_70(_a, _b.as_pointer())
    }) {
        let flexLine: Value<facebook_yoga_FlexLine> = Rc::new(RefCell::new(
            ({
                calculateFlexLine_467(
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
        let mut canSkipFlex: () =
            (!(performLayout)) && (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit);
        let mut sizeBasedOnContent: () = false;
        if (sizingModeMainDim != facebook_yoga_SizingMode_StretchFit) {
            let style: Ptr<facebook_yoga_Style> = ({ facebook_yoga_NodeImpl::style_24(&node) });
            let mut minInnerWidth: () = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: () = ownerWidth;
                                let _ownerWidth: () = ownerWidth;
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
            let mut maxInnerWidth: () = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: () = ownerWidth;
                                let _ownerWidth: () = ownerWidth;
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
            let mut minInnerHeight: () = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: () = ownerHeight;
                                let _ownerWidth: () = ownerWidth;
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
            let mut maxInnerHeight: () = ({
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _direction: facebook_yoga_Direction = direction;
                                let _referenceLength: () = ownerHeight;
                                let _ownerWidth: () = ownerWidth;
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
            let mut minInnerMainDim: () = if isMainAxisRow {
                minInnerWidth
            } else {
                minInnerHeight
            };
            let mut maxInnerMainDim: () = if isMainAxisRow {
                maxInnerWidth
            } else {
                maxInnerHeight
            };
            if ({ isDefined_101(minInnerMainDim) })
                && ({ (*flexLine.borrow()).sizeConsumed } < minInnerMainDim)
            {
                availableInnerMainDim = minInnerMainDim;
            } else if ({ isDefined_101(maxInnerMainDim) })
                && ({ (*flexLine.borrow()).sizeConsumed } > maxInnerMainDim)
            {
                availableInnerMainDim = maxInnerMainDim;
            } else {
                let mut useLegacyStretchBehaviour: () = ({
                    facebook_yoga_NodeImpl::hasErrata(&node, facebook_yoga_Errata_StretchFlexBasis)
                });
                if (!(useLegacyStretchBehaviour))
                    && ((({ isDefined_101({ (*flexLine.borrow()).layout.totalFlexGrowFactors }) })
                        && ({ (*flexLine.borrow()).layout.totalFlexGrowFactors } == 0_()))
                        || (({
                            isDefined_101(({ facebook_yoga_NodeImpl::resolveFlexGrow(&node) }))
                        }) && (({ facebook_yoga_NodeImpl::resolveFlexGrow(&node) }) == 0_())))
                {
                    availableInnerMainDim = { (*flexLine.borrow()).sizeConsumed };
                }
                sizeBasedOnContent = !(useLegacyStretchBehaviour);
            }
        }
        if (!(sizeBasedOnContent)) && ({ isDefined_101(availableInnerMainDim) }) {
            (*flexLine.borrow_mut()).layout.remainingFreeSpace =
                { (availableInnerMainDim - { (*flexLine.borrow()).sizeConsumed }) };
        } else if ({ (*flexLine.borrow()).sizeConsumed } < 0_()) {
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
                let _ownerWidth: () = ownerWidth;
                let _mainAxisOwnerSize: () = mainAxisOwnerSize;
                let _availableInnerMainDim: () = availableInnerMainDim;
                let _availableInnerCrossDim: () = availableInnerCrossDim;
                let _availableInnerWidth: () = availableInnerWidth;
                let _availableInnerHeight: () = availableInnerHeight;
                let _mainAxisOverflows: () = mainAxisOverflows;
                let _sizingModeCrossDim: facebook_yoga_SizingMode = sizingModeCrossDim;
                let _performLayout: () = performLayout;
                let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
                let _depth: () = depth;
                let _generationCount: () = generationCount;
                resolveFlexibleLength_464(
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
            let _hadOverflow: () = ({
                facebook_yoga_LayoutResultsImpl::hadOverflow(
                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                )
            }) || ({ (*flexLine.borrow()).layout.remainingFreeSpace }
                < 0_());
            facebook_yoga_NodeImpl::setLayoutHadOverflow(&node, _hadOverflow)
        });
        ({
            justifyMainAxis_465(
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
        let mut containerCrossAxis: () = availableInnerCrossDim;
        if (sizingModeCrossDim == facebook_yoga_SizingMode_MaxContent)
            || (sizingModeCrossDim == facebook_yoga_SizingMode_FitContent)
        {
            containerCrossAxis = ({
                ({
                    boundAxis_420(
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
                    boundAxis_420(
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
            'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
                let mut leadingCrossDim: () = leadingPaddingAndBorderCross;
                let mut alignItem: facebook_yoga_Align =
                    ({ resolveChildAlignment_414((node).clone(), (child).clone()) });
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
                            ({ dimension_131(crossAxis) }),
                            availableInnerCrossDim,
                        )
                    }) {
                        let childMainSize: Value<()> = Rc::new(RefCell::new(
                            ({
                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                    ({ dimension_131(mainAxis) }),
                                )
                            }),
                        ));
                        let childStyle: Ptr<facebook_yoga_Style> =
                            ({ facebook_yoga_NodeImpl::style_24(&child) });
                        let childCrossSize: Value<()> = Rc::new(RefCell::new(
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
                            constrainMaxSizeForMode_450(
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
                            constrainMaxSizeForMode_450(
                                (child).clone(),
                                direction,
                                crossAxis,
                                availableInnerCrossDim,
                                availableInnerWidth,
                                (childCrossSizingMode.as_pointer()),
                                (childCrossSize.as_pointer()),
                            )
                        });
                        let mut childWidth: () = if isMainAxisRow {
                            (*childMainSize.borrow())
                        } else {
                            (*childCrossSize.borrow())
                        };
                        let mut childHeight: () = if !(isMainAxisRow) {
                            (*childMainSize.borrow())
                        } else {
                            (*childCrossSize.borrow())
                        };
                        let mut alignContent: facebook_yoga_Align = ({
                            facebook_yoga_StyleImpl::alignContent(
                                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                            )
                        });
                        let mut crossAxisDoesNotGrow: () =
                            (alignContent != facebook_yoga_Align_Stretch) && (isNodeFlexWrap);
                        let mut childWidthSizingMode: facebook_yoga_SizingMode =
                            if ({ isUndefined_99(childWidth) })
                                || ((!(isMainAxisRow)) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        let mut childHeightSizingMode: facebook_yoga_SizingMode =
                            if ({ isUndefined_99(childHeight) })
                                || ((isMainAxisRow) && (crossAxisDoesNotGrow))
                            {
                                facebook_yoga_SizingMode_MaxContent
                            } else {
                                facebook_yoga_SizingMode_StretchFit
                            };
                        ({
                            let _node: Ptr<facebook_yoga_Node> = (child).clone();
                            let _availableWidth: () = childWidth;
                            let _availableHeight: () = childHeight;
                            let _ownerDirection: facebook_yoga_Direction = direction;
                            let _widthSizingMode: facebook_yoga_SizingMode = childWidthSizingMode;
                            let _heightSizingMode: facebook_yoga_SizingMode = childHeightSizingMode;
                            let _ownerWidth: () = availableInnerWidth;
                            let _ownerHeight: () = availableInnerHeight;
                            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> =
                                (layoutMarkerData).clone();
                            let _depth: () = depth;
                            let _generationCount: () = generationCount;
                            calculateLayoutInternal_431(
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
                    let mut remainingCrossDim: () = ({ containerCrossAxis } - {
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
                            ({ maxOrDefined_104(0_f32, (remainingCrossDim / 2_())) });
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
                        leadingCrossDim += ({ maxOrDefined_104(0_f32, remainingCrossDim) });
                    } else if (alignItem == facebook_yoga_Align_FlexStart) {
                    } else if (alignItem == facebook_yoga_Align_Center) {
                        leadingCrossDim += (remainingCrossDim / 2_());
                    } else {
                        leadingCrossDim += remainingCrossDim;
                    }
                }
                ({
                    let _position: () = ({
                        ({
                            ({
                                facebook_yoga_LayoutResultsImpl::position(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                    ({ flexStartEdge_127(crossAxis) }),
                                )
                            })
                        } + { totalLineCrossDim })
                    } + { leadingCrossDim });
                    let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_127(crossAxis) });
                    facebook_yoga_NodeImpl::setLayoutPosition(&child, _position, _edge)
                });
            }
        }
        let mut appliedCrossGap: () = if (lineCount != 0_()) {
            crossAxisGap
        } else {
            0_f32
        };
        totalLineCrossDim += ({ (*flexLine.borrow()).layout.crossDim } + appliedCrossGap);
        let __rhs = ({ maxOrDefined_104(maxLineMainDim, { (*flexLine.borrow()).layout.mainDim }) });
        maxLineMainDim = __rhs;
        lineCount.postfix_inc();
    }
    if (performLayout) && ((isNodeFlexWrap) || ({ isBaselineLayout_435((node).clone()) })) {
        let mut leadPerLine: () = 0_();
        let mut currentLead: () = leadingPaddingAndBorderCross;
        let mut extraSpacePerLine: () = 0_();
        let mut unclampedCrossDim: () =
            if (sizingModeCrossDim == facebook_yoga_SizingMode_StretchFit) {
                (availableInnerCrossDim + paddingAndBorderAxisCross)
            } else {
                if ({
                    facebook_yoga_NodeImpl::hasDefiniteLength(
                        &node,
                        ({ dimension_131(crossAxis) }),
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
                                        ({ dimension_131(crossAxis) }),
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
        let mut innerCrossDim: () = ({
            ({
                boundAxis_420(
                    (node).clone(),
                    crossAxis,
                    direction,
                    unclampedCrossDim,
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            })
        } - { paddingAndBorderAxisCross });
        let mut remainingAlignContentDim: () = (innerCrossDim - totalLineCrossDim);
        let mut alignContent: facebook_yoga_Align = if (remainingAlignContentDim >= 0_()) {
            ({
                facebook_yoga_StyleImpl::alignContent(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            })
        } else {
            ({
                fallbackAlignment_416(
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
                    currentLead += (remainingAlignContentDim / 2_());
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_Stretch => {
                    extraSpacePerLine = (remainingAlignContentDim / lineCount);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceAround => {
                    currentLead += (remainingAlignContentDim / (2_() * lineCount));
                    leadPerLine = (remainingAlignContentDim / lineCount);
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceEvenly => {
                    currentLead += (remainingAlignContentDim / (lineCount).wrapping_add(1_()));
                    leadPerLine = (remainingAlignContentDim / (lineCount).wrapping_add(1_()));
                    break 'switch;
                }
                __v if __v == facebook_yoga_Align_SpaceBetween => {
                    if (lineCount > 1_()) {
                        leadPerLine = (remainingAlignContentDim / (lineCount).wrapping_sub(1_()));
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
        let mut i: () = 0_();
        'loop_: while (i < lineCount) {
            let startIterator: Value<
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator,
            > = Rc::new(RefCell::new((*endIterator.borrow()).clone()));
            let iterator: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new((*startIterator.borrow()).clone()));
            let mut lineHeight: () = 0_();
            let mut maxAscentForCurrentLine: () = 0_();
            let mut maxDescentForCurrentLine: () = 0_();
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
                operator_ne_70(_a, _b.as_pointer())
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
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0_()  , )
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
                            maxOrDefined_104(
                                lineHeight,
                                ({
                                    ({
                                        facebook_yoga_LayoutResultsImpl::measuredDimension(
                                            &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                            ({ dimension_131(crossAxis) }),
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
                    if (({ resolveChildAlignment_414((node).clone(), (child).clone()) })
                        == facebook_yoga_Align_Baseline)
                    {
                        let mut ascent: () = ({ ({ calculateBaseline_434((child).clone()) }) } + {
                            ({
                                facebook_yoga_StyleImpl::computeFlexStartMargin(
                                    &({ facebook_yoga_NodeImpl::style_24(&child) }),
                                    facebook_yoga_FlexDirection_Column,
                                    direction,
                                    availableInnerWidth,
                                )
                            })
                        });
                        let mut descent: () = ({
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
                        let __rhs = ({ maxOrDefined_104(maxAscentForCurrentLine, ascent) });
                        maxAscentForCurrentLine = __rhs;
                        let __rhs = ({ maxOrDefined_104(maxDescentForCurrentLine, descent) });
                        maxDescentForCurrentLine = __rhs;
                        let __rhs = ({
                            maxOrDefined_104(
                                lineHeight,
                                (maxAscentForCurrentLine + maxDescentForCurrentLine),
                            )
                        });
                        lineHeight = __rhs;
                    }
                }
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0_()  , )
                });
            }
            (*endIterator.borrow_mut()) = (*iterator.borrow()).clone();
            currentLead += if (i != 0_()) { crossAxisGap } else { 0_() };
            lineHeight += extraSpacePerLine;
            (*iterator.borrow_mut()) = (*startIterator.borrow()).clone();
            'loop_: while ({
                let _a: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                    iterator.as_pointer();
                operator_ne_70(_a, endIterator.as_pointer())
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
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0_()  , )
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
                        match { ({ resolveChildAlignment_414((node).clone(), (child).clone()) }) } {
                            __v if __v == facebook_yoga_Align_Start
                                || __v == facebook_yoga_Align_End =>
                            {
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexStart => {
                                ({
                                    let _position: () = ({ currentLead } + {
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
                                        ({ flexStartEdge_127(crossAxis) });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child, _position, _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_FlexEnd => {
                                ({
                                    let _position: () = ({
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
                                                ({ dimension_131(crossAxis) }),
                                            )
                                        })
                                    });
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ flexStartEdge_127(crossAxis) });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child, _position, _edge,
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Center => {
                                let mut childHeight: () = ({
                                    facebook_yoga_LayoutResultsImpl::measuredDimension(
                                        &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                        ({ dimension_131(crossAxis) }),
                                    )
                                });
                                ({
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child,
                                        (currentLead + ((lineHeight - childHeight) / 2_())),
                                        ({ flexStartEdge_127(crossAxis) }),
                                    )
                                });
                                break 'switch;
                            }
                            __v if __v == facebook_yoga_Align_Stretch => {
                                ({
                                    let _position: () = ({ currentLead } + {
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
                                        ({ flexStartEdge_127(crossAxis) });
                                    facebook_yoga_NodeImpl::setLayoutPosition(
                                        &child, _position, _edge,
                                    )
                                });
                                if !({
                                    facebook_yoga_NodeImpl::hasDefiniteLength(
                                        &child,
                                        ({ dimension_131(crossAxis) }),
                                        availableInnerCrossDim,
                                    )
                                }) {
                                    let mut childWidth: () = if isMainAxisRow {
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
                                    let mut childHeight: () = if !(isMainAxisRow) {
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
                                        inexactEquals_105(
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
                                        inexactEquals_105(
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
                                            let _availableWidth: () = childWidth;
                                            let _availableHeight: () = childHeight;
                                            let _ownerDirection: facebook_yoga_Direction =
                                                direction;
                                            let _ownerWidth: () = availableInnerWidth;
                                            let _ownerHeight: () = availableInnerHeight;
                                            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> =
                                                (layoutMarkerData).clone();
                                            let _depth: () = depth;
                                            let _generationCount: () = generationCount;
                                            calculateLayoutInternal_431(
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
                                    let _position: () = ({
                                        ({ (currentLead + maxAscentForCurrentLine) } - {
                                            ({ calculateBaseline_434((child).clone()) })
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
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator .as_pointer()  , 0_()  , )
                });
            }
            currentLead = { ((currentLead + leadPerLine) + lineHeight) };
            i.postfix_inc();
        }
    }
    ({
        let _measuredDimension: () = ({
            let _axisSize: () = ownerWidth;
            let _widthSize: () = ownerWidth;
            boundAxis_420(
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
        let _measuredDimension: () = ({
            boundAxis_420(
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
            let _measuredDimension: () = ({
                boundAxis_420(
                    (node).clone(),
                    mainAxis,
                    direction,
                    maxLineMainDim,
                    mainAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_131(mainAxis) });
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
            let _measuredDimension: () = ({
                let _a: () = ({
                    minOrDefined_439(
                        (availableInnerMainDim + paddingAndBorderAxisMain),
                        ({
                            facebook_yoga_FloatOptionalImpl::unwrap(
                                &Rc::new(RefCell::new(
                                    ({
                                        boundAxisWithinMinAndMax_419(
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
                let _b: () = paddingAndBorderAxisMain;
                maxOrDefined_104(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_131(mainAxis) });
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
            let _measuredDimension: () = ({
                boundAxis_420(
                    (node).clone(),
                    crossAxis,
                    direction,
                    (totalLineCrossDim + paddingAndBorderAxisCross),
                    crossAxisOwnerSize,
                    ownerWidth,
                )
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_131(crossAxis) });
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
            let _measuredDimension: () = ({
                let _a: () = ({
                    let _a: () = (availableInnerCrossDim + paddingAndBorderAxisCross);
                    let _b: () = ({
                        facebook_yoga_FloatOptionalImpl::unwrap(
                            &Rc::new(RefCell::new(
                                ({
                                    boundAxisWithinMinAndMax_419(
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
                    minOrDefined_439(_a, _b)
                });
                let _b: () = paddingAndBorderAxisCross;
                maxOrDefined_104(_a, _b)
            });
            let _dimension: facebook_yoga_Dimension = ({ dimension_131(crossAxis) });
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
            operator_ne_70(_a, __end3.as_pointer())
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
                    let _position: () = ({
                        ({
                            ({
                                facebook_yoga_LayoutResultsImpl::measuredDimension(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                                    ({ dimension_131(crossAxis) }),
                                )
                            })
                        } - {
                            ({
                                facebook_yoga_LayoutResultsImpl::position(
                                    &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                    ({ flexStartEdge_127(crossAxis) }),
                                )
                            })
                        })
                    } - {
                        ({
                            facebook_yoga_LayoutResultsImpl::measuredDimension(
                                &({ facebook_yoga_NodeImpl::getLayout_26(&child) }),
                                ({ dimension_131(crossAxis) }),
                            )
                        })
                    });
                    let _edge: facebook_yoga_PhysicalEdge = ({ flexStartEdge_127(crossAxis) });
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
        let mut needsMainTrailingPos: () = ({ needsTrailingPosition_423(mainAxis) });
        let mut needsCrossTrailingPos: () = ({ needsTrailingPosition_423(crossAxis) });
        if (needsMainTrailingPos) || (needsCrossTrailingPos) {
            let __tmp_2: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> = Rc::new(
                RefCell::new(({ facebook_yoga_NodeImpl::getLayoutChildren(&node) })),
            );
            let __range4: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node_> =
                __tmp_2.as_pointer();
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
                operator_ne_70(_a, __end4.as_pointer())
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
                    ({ setChildTrailingPosition_422((node).clone(), (child).clone(), mainAxis) });
                }
                if needsCrossTrailingPos {
                    ({ setChildTrailingPosition_422((node).clone(), (child).clone(), crossAxis) });
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
            || (depth == 1_())
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
                let _currentDepth: () = depth;
                let _generationCount: () = generationCount;
                let _containingNodeAvailableInnerWidth: () = availableInnerWidth;
                let _containingNodeAvailableInnerHeight: () = availableInnerHeight;
                layoutAbsoluteDescendants_432(
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
pub fn isCachedForOwnerSize_468(
    mut node: Ptr<facebook_yoga_Node>,
    cachedMeasurement: Ptr<facebook_yoga_CachedMeasurement>,
    mut ownerWidth: (),
    mut ownerHeight: (),
) -> () {
    return ((!({
        facebook_yoga_StyleImpl::resolvesAgainstOwnerWidth(
            &({ facebook_yoga_NodeImpl::style_25(&node) }),
        )
    })) || ({
        inexactEquals_105(cachedMeasurement.with(|__s| __s.ownerWidth), ownerWidth)
    })) && ((!({
        facebook_yoga_StyleImpl::resolvesAgainstOwnerHeight(
            &({ facebook_yoga_NodeImpl::style_25(&node) }),
        )
    })) || ({
        inexactEquals_105(cachedMeasurement.with(|__s| __s.ownerHeight), ownerHeight)
    }));
}
pub fn calculateLayoutInternal_431(
    mut node: Ptr<facebook_yoga_Node>,
    mut availableWidth: (),
    mut availableHeight: (),
    mut ownerDirection: facebook_yoga_Direction,
    mut widthSizingMode: facebook_yoga_SizingMode,
    mut heightSizingMode: facebook_yoga_SizingMode,
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut performLayout: (),
    mut reason: facebook_yoga_LayoutPassReason,
    layoutMarkerData: Ptr<facebook_yoga_LayoutData>,
    mut depth: (),
    mut generationCount: (),
) -> () {
    let mut layout: Ptr<facebook_yoga_LayoutResults> =
        ({ facebook_yoga_NodeImpl::getLayout_26(&node) });
    depth.postfix_inc();
    let mut needToVisitNode: () = ((({ facebook_yoga_NodeImpl::isDirty(&node) })
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
        field!(layout, nextCachedMeasurementsIndex).write(0_());
        field!(field!(layout, cachedLayout), availableWidth).write(-1_());
        field!(field!(layout, cachedLayout), availableHeight).write(-1_());
        field!(field!(layout, cachedLayout), widthSizingMode)
            .write(facebook_yoga_SizingMode_MaxContent);
        field!(field!(layout, cachedLayout), heightSizingMode)
            .write(facebook_yoga_SizingMode_MaxContent);
        field!(field!(layout, cachedLayout), computedWidth).write(-1_());
        field!(field!(layout, cachedLayout), computedHeight).write(-1_());
        field!(field!(layout, cachedLayout), ownerWidth).write(-1_());
        field!(field!(layout, cachedLayout), ownerHeight).write(-1_());
    }
    let mut cachedResults: Ptr<facebook_yoga_CachedMeasurement> =
        Ptr::<facebook_yoga_CachedMeasurement>::null();
    if ({ facebook_yoga_NodeImpl::hasMeasureFunc(&node) }) {
        let mut marginAxisRow: () = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                facebook_yoga_FlexDirection_Row,
                ownerWidth,
            )
        });
        let mut marginAxisColumn: () = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&node) }),
                facebook_yoga_FlexDirection_Column,
                ownerWidth,
            )
        });
        if ({
            let _lastWidthMode: facebook_yoga_SizingMode =
                layout.with(|__s| __s.cachedLayout.widthSizingMode);
            let _lastAvailableWidth: () = layout.with(|__s| __s.cachedLayout.availableWidth);
            let _lastHeightMode: facebook_yoga_SizingMode =
                layout.with(|__s| __s.cachedLayout.heightSizingMode);
            let _lastAvailableHeight: () = layout.with(|__s| __s.cachedLayout.availableHeight);
            let _lastComputedWidth: () = layout.with(|__s| __s.cachedLayout.computedWidth);
            let _lastComputedHeight: () = layout.with(|__s| __s.cachedLayout.computedHeight);
            canUseCachedMeasurement_243(
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
            isCachedForOwnerSize_468(
                (node).clone(),
                field_ptr!(layout, cachedLayout),
                ownerWidth,
                ownerHeight,
            )
        }) {
            cachedResults = (field_ptr!(layout, cachedLayout));
        } else {
            let mut i: () = 0_();
            'loop_: while ({ i } < { layout.with(|__s| __s.nextCachedMeasurementsIndex) }) {
                if ({
                    let _lastWidthMode: facebook_yoga_SizingMode =
                        unimplemented!().with(|__s| __s.widthSizingMode);
                    let _lastAvailableWidth: () = unimplemented!().with(|__s| __s.availableWidth);
                    let _lastHeightMode: facebook_yoga_SizingMode =
                        unimplemented!().with(|__s| __s.heightSizingMode);
                    let _lastAvailableHeight: () = unimplemented!().with(|__s| __s.availableHeight);
                    let _lastComputedWidth: () = unimplemented!().with(|__s| __s.computedWidth);
                    let _lastComputedHeight: () = unimplemented!().with(|__s| __s.computedHeight);
                    canUseCachedMeasurement_243(
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
                    isCachedForOwnerSize_468(
                        (node).clone(),
                        unimplemented!(),
                        ownerWidth,
                        ownerHeight,
                    )
                }) {
                    cachedResults = (unimplemented!());
                    break;
                }
                i.postfix_inc();
            }
        }
    } else if performLayout {
        if (((({
            inexactEquals_105(
                layout.with(|__s| __s.cachedLayout.availableWidth),
                availableWidth,
            )
        }) && ({
            inexactEquals_105(
                layout.with(|__s| __s.cachedLayout.availableHeight),
                availableHeight,
            )
        })) && ({ layout.with(|__s| __s.cachedLayout.widthSizingMode) } == {
            widthSizingMode
        })) && ({ layout.with(|__s| __s.cachedLayout.heightSizingMode) } == {
            heightSizingMode
        })) && ({
            isCachedForOwnerSize_468(
                (node).clone(),
                field_ptr!(layout, cachedLayout),
                ownerWidth,
                ownerHeight,
            )
        }) {
            cachedResults = (field_ptr!(layout, cachedLayout));
        }
    } else {
        let mut i: () = 0_();
        'loop_: while ({ i } < { layout.with(|__s| __s.nextCachedMeasurementsIndex) }) {
            if (((({
                inexactEquals_105(
                    unimplemented!().with(|__s| __s.availableWidth),
                    availableWidth,
                )
            }) && ({
                inexactEquals_105(
                    unimplemented!().with(|__s| __s.availableHeight),
                    availableHeight,
                )
            })) && ({ unimplemented!().with(|__s| __s.widthSizingMode) } == {
                widthSizingMode
            })) && ({ unimplemented!().with(|__s| __s.heightSizingMode) } == {
                heightSizingMode
            })) && ({
                isCachedForOwnerSize_468((node).clone(), unimplemented!(), ownerWidth, ownerHeight)
            }) {
                cachedResults = (unimplemented!());
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
            .with_mut(|__v| *__v = *__v + 1_())
        };
    } else {
        ({
            let _node: Ptr<facebook_yoga_Node> = (node).clone();
            let _availableWidth: () = availableWidth;
            let _availableHeight: () = availableHeight;
            let _ownerDirection: facebook_yoga_Direction = ownerDirection;
            let _widthSizingMode: facebook_yoga_SizingMode = widthSizingMode;
            let _heightSizingMode: facebook_yoga_SizingMode = heightSizingMode;
            let _ownerWidth: () = ownerWidth;
            let _ownerHeight: () = ownerHeight;
            let _performLayout: () = performLayout;
            let _reason: facebook_yoga_LayoutPassReason = reason;
            let _layoutMarkerData: Ptr<facebook_yoga_LayoutData> = (layoutMarkerData).clone();
            let _depth: () = depth;
            let _generationCount: () = generationCount;
            calculateLayoutImpl_466(
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
            let __rhs = (unimplemented!().read());
            field!(layoutMarkerData, maxMeasureCache).write(__rhs);
            if ({ layout.with(|__s| __s.nextCachedMeasurementsIndex) } == {
                MaxCachedMeasurements_117.with(|rc| *rc.borrow())
            }) {
                field!(layout, nextCachedMeasurementsIndex).write(0_());
            }
            let mut newCacheEntry: Ptr<facebook_yoga_CachedMeasurement> =
                Ptr::<facebook_yoga_CachedMeasurement>::null();
            if performLayout {
                newCacheEntry = (field_ptr!(layout, cachedLayout));
            } else {
                newCacheEntry = (unimplemented!());
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
            let _lengthValue: () = ({
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
            let _lengthValue: () = ({
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
        facebook_yoga_Event::publish_3_2((node).clone(), _eventData.as_pointer())
    });
    return ((needToVisitNode) || ((cachedResults).is_null()));
}
pub fn calculateLayout_209(
    mut node: Ptr<facebook_yoga_Node>,
    mut ownerWidth: (),
    mut ownerHeight: (),
    mut ownerDirection: facebook_yoga_Direction,
) {
    ({ facebook_yoga_Event::publish_3_3((node).clone(), None) });
    let markerData: Value<facebook_yoga_LayoutData> =
        Rc::new(RefCell::new(<facebook_yoga_LayoutData>::default()));
    let mut currentGenerationCount: () = (unimplemented!()).wrapping_add(1_());
    ({ facebook_yoga_NodeImpl::processDimensions(&node) });
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    let mut width: () = YGUndefined_24.with(|rc| *rc.borrow());
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
                            let _referenceLength: () = ownerWidth;
                            let _ownerWidth: () = ownerWidth;
                            facebook_yoga_NodeImpl::getResolvedDimension(
                                &node,
                                direction,
                                ({ dimension_131(facebook_yoga_FlexDirection_Row) }),
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
                    let _referenceLength: () = ownerWidth;
                    let _ownerWidth: () = ownerWidth;
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
                        let _referenceLength: () = ownerWidth;
                        let _ownerWidth: () = ownerWidth;
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
        widthSizingMode = if ({ isUndefined_99(width) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let mut height: () = YGUndefined_24.with(|rc| *rc.borrow());
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
                                ({ dimension_131(facebook_yoga_FlexDirection_Column) }),
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
                    let _referenceLength: () = ownerHeight;
                    let _ownerWidth: () = ownerWidth;
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
                        let _referenceLength: () = ownerHeight;
                        let _ownerWidth: () = ownerWidth;
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
        heightSizingMode = if ({ isUndefined_99(height) }) {
            facebook_yoga_SizingMode_MaxContent
        } else {
            facebook_yoga_SizingMode_StretchFit
        };
    }
    let mut generationCount: () = if ({
        facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
            facebook_yoga_ExperimentalFeature_FixFlexBasisFitContent,
        )
    }) {
        currentGenerationCount
    } else {
        unimplemented!()
    };
    if ({
        calculateLayoutInternal_431(
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
            0_(),
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
        ({ roundLayoutResultsToPixelGrid_469((node).clone(), 0_f32, 0_f32) });
    }
    ({
        let _eventData: Value<facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_> =
            Rc::new(RefCell::new(
                facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassEnd_ {
                    layoutData: (markerData.as_pointer()),
                },
            ));
        facebook_yoga_Event::publish_3_4((node).clone(), _eventData.as_pointer())
    });
}
pub fn calculateFlexLine_467(
    mut node: Ptr<facebook_yoga_Node>,
    mut ownerDirection: facebook_yoga_Direction,
    mut ownerWidth: (),
    mut mainAxisOwnerSize: (),
    mut availableInnerWidth: (),
    mut availableInnerMainDim: (),
    iterator: Ptr<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator>,
    mut lineCount: (),
) -> facebook_yoga_FlexLine {
    let mut itemsInFlow: () = unimplemented!();
    unimplemented!();
    let mut sizeConsumed: () = 0_f32;
    let mut totalFlexGrowFactors: () = 0_f32;
    let mut totalFlexShrinkScaledFactors: () = 0_f32;
    let mut numberOfAutoMargins: () = 0_();
    let mut firstElementInLine: Ptr<facebook_yoga_Node> = Ptr::<facebook_yoga_Node>::null();
    let mut sizeConsumedIncludingMinConstraint: () = 0_();
    let mut direction: facebook_yoga_Direction =
        ({ facebook_yoga_NodeImpl::resolveDirection(&node, ownerDirection) });
    let mut mainAxis: facebook_yoga_FlexDirection = ({
        resolveDirection_125(
            ({
                facebook_yoga_StyleImpl::flexDirection(
                    &({ facebook_yoga_NodeImpl::style_24(&node) }),
                )
            }),
            direction,
        )
    });
    let mut isNodeFlexWrap: () =
        (({ facebook_yoga_StyleImpl::flexWrap(&({ facebook_yoga_NodeImpl::style_24(&node) })) })
            != facebook_yoga_Wrap_NoWrap);
    let mut gap: () = ({
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
        operator_ne_70(_a, _b)
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
                facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iterator  , 0_()  , )
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
        let mut childMarginMainAxis: () = ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &({ facebook_yoga_NodeImpl::style_24(&child) }),
                mainAxis,
                availableInnerWidth,
            )
        });
        let mut childLeadingGapMainAxis: () =
            if ({ (child).clone() } == { (firstElementInLine).clone() }) {
                0_f32
            } else {
                gap
            };
        let mut flexBasisWithMinAndMaxConstraints: () = ({
            facebook_yoga_FloatOptionalImpl::unwrap(
                &Rc::new(RefCell::new(
                    ({
                        let _node: Ptr<facebook_yoga_Node> = (child).clone();
                        let _value: facebook_yoga_FloatOptional =
                            ({ facebook_yoga_NodeImpl::getLayout_26(&child) })
                                .with(|__s| __s.computedFlexBasis.clone());
                        boundAxisWithinMinAndMax_419(
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
        let mut requiredMainDim: () = (((sizeConsumedIncludingMinConstraint
            + flexBasisWithMinAndMaxConstraints)
            + childMarginMainAxis)
            + childLeadingGapMainAxis);
        if (((requiredMainDim > availableInnerMainDim)
            && (!({ inexactEquals_105(requiredMainDim, availableInnerMainDim) })))
            && (isNodeFlexWrap))
            && (!(unimplemented!()))
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
        unimplemented!();
        ({
            facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::operator_post_inc_5(
                &iterator,
                0_(),
            )
        });
    }
    if (totalFlexGrowFactors > 0_()) && (totalFlexGrowFactors < 1_()) {
        totalFlexGrowFactors = 1_();
    }
    if (totalFlexShrinkScaledFactors > 0_()) && (totalFlexShrinkScaledFactors < 1_()) {
        totalFlexShrinkScaledFactors = 1_();
    }
    return facebook_yoga_FlexLine {
        itemsInFlow: unimplemented!(),
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
pub fn roundValueToPixelGridDouble_470(
    mut value: (),
    mut pointScaleFactor: (),
    mut forceCeil: (),
    mut forceFloor: (),
) -> () {
    let mut scaledValue: () = (value * pointScaleFactor);
    let mut fractional: () = unimplemented!();
    if (fractional < 0_()) {
        fractional.prefix_inc();
    }
    if ({ inexactEquals_106(fractional, 0_()) }) {
        scaledValue = { (scaledValue - fractional) };
    } else if ({ inexactEquals_106(fractional, 1_f64) }) {
        scaledValue = { ((scaledValue - fractional) + 1_f64) };
    } else if forceCeil {
        scaledValue = { ((scaledValue - fractional) + 1_f64) };
    } else if forceFloor {
        scaledValue = { (scaledValue - fractional) };
    } else {
        let __rhs = ((scaledValue - fractional)
            + (if (!(unimplemented!()))
                && ((fractional > 0.5_f64) || ({ inexactEquals_106(fractional, 0.5_f64) }))
            {
                1_f64
            } else {
                0_f64
            }));
        scaledValue = __rhs;
    }
    return if ((unimplemented!()) || (unimplemented!())) {
        YGUndefined_24.with(|rc| *rc.borrow())
    } else {
        (scaledValue / pointScaleFactor)
    };
}
pub fn roundValueToPixelGrid_412(
    mut value: (),
    mut pointScaleFactor: (),
    mut forceCeil: (),
    mut forceFloor: (),
) -> () {
    return ({ roundValueToPixelGridDouble_470(value, pointScaleFactor, forceCeil, forceFloor) });
}
pub fn roundLayoutResultsToPixelGrid_469(
    mut node: Ptr<facebook_yoga_Node>,
    mut absoluteLeft: (),
    mut absoluteTop: (),
) {
    let mut pointScaleFactor: () = ({
        facebook_yoga_ConfigImpl::getPointScaleFactor(
            &({ facebook_yoga_NodeImpl::getConfig(&node) }),
        )
    });
    let mut nodeLeft: () = ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_PhysicalEdge_Left,
        )
    });
    let mut nodeTop: () = ({
        facebook_yoga_LayoutResultsImpl::position(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_PhysicalEdge_Top,
        )
    });
    let mut nodeWidth: () = ({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_Dimension_Width,
        )
    });
    let mut nodeHeight: () = ({
        facebook_yoga_LayoutResultsImpl::dimension(
            &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
            facebook_yoga_Dimension_Height,
        )
    });
    let mut absoluteNodeLeft: () = (absoluteLeft + nodeLeft);
    let mut absoluteNodeTop: () = (absoluteTop + nodeTop);
    let mut absoluteNodeRight: () = (absoluteNodeLeft + nodeWidth);
    let mut absoluteNodeBottom: () = (absoluteNodeTop + nodeHeight);
    if (pointScaleFactor != 0_f64) {
        let mut textRounding: () =
            (({ facebook_yoga_NodeImpl::getNodeType(&node) }) == facebook_yoga_NodeType_Text);
        ({
            facebook_yoga_NodeImpl::setLayoutPosition(
                &node,
                ({ roundValueToPixelGrid_412(nodeLeft, pointScaleFactor, false, textRounding) }),
                facebook_yoga_PhysicalEdge_Left,
            )
        });
        ({
            facebook_yoga_NodeImpl::setLayoutPosition(
                &node,
                ({ roundValueToPixelGrid_412(nodeTop, pointScaleFactor, false, textRounding) }),
                facebook_yoga_PhysicalEdge_Top,
            )
        });
        let mut scaledNodeWidth: () = (nodeWidth * pointScaleFactor);
        let mut hasFractionalWidth: () = !({
            let _a: () = unimplemented!();
            let _b: () = scaledNodeWidth;
            inexactEquals_106(_a, _b)
        });
        let mut scaledNodeHeight: () = (nodeHeight * pointScaleFactor);
        let mut hasFractionalHeight: () = !({
            let _a: () = unimplemented!();
            let _b: () = scaledNodeHeight;
            inexactEquals_106(_a, _b)
        });
        ({
            facebook_yoga_LayoutResultsImpl::setDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                facebook_yoga_Dimension_Width,
                if (textRounding) && (!(hasFractionalWidth)) {
                    (unimplemented!() / pointScaleFactor)
                } else {
                    (({
                        roundValueToPixelGridDouble_470(
                            absoluteNodeRight,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - ({
                        roundValueToPixelGridDouble_470(
                            absoluteNodeLeft,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    }))
                },
            )
        });
        ({
            facebook_yoga_LayoutResultsImpl::setDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(&node) }),
                facebook_yoga_Dimension_Height,
                if (textRounding) && (!(hasFractionalHeight)) {
                    (unimplemented!() / pointScaleFactor)
                } else {
                    (({
                        roundValueToPixelGridDouble_470(
                            absoluteNodeBottom,
                            pointScaleFactor,
                            textRounding,
                            false,
                        )
                    }) - ({
                        roundValueToPixelGridDouble_470(
                            absoluteNodeTop,
                            pointScaleFactor,
                            false,
                            textRounding,
                        )
                    }))
                },
            )
        });
    }
    let __range2: Ptr<()> = ({ facebook_yoga_NodeImpl::getChildren(&node) });
    'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut child: Ptr<facebook_yoga_Node> = (unimplemented!().read());
        if ({ ({ facebook_yoga_NodeImpl::getOwner(&child) }) } != { (node).clone() }) {
            continue 'loop_;
        }
        ({ roundLayoutResultsToPixelGrid_469((child).clone(), absoluteNodeLeft, absoluteNodeTop) });
    }
}
pub fn configUpdateInvalidatesLayout_471(
    oldConfig: Ptr<facebook_yoga_Config>,
    newConfig: Ptr<facebook_yoga_Config>,
) -> () {
    return ((({ ({ facebook_yoga_ConfigImpl::getErrata(&oldConfig) }) } != {
        ({ facebook_yoga_ConfigImpl::getErrata(&newConfig) })
    }) || (!(unimplemented!())))
        || ({ ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&oldConfig) }) } != {
            ({ facebook_yoga_ConfigImpl::getPointScaleFactor(&newConfig) })
        }))
        || ({ ({ facebook_yoga_ConfigImpl::useWebDefaults(&oldConfig) }) } != {
            ({ facebook_yoga_ConfigImpl::useWebDefaults(&newConfig) })
        });
}
pub fn fatalWithMessage_97(mut message: Ptr<()>) {
    unimplemented!();
}
pub fn assertFatal_202(mut condition: (), mut message: Ptr<()>) {
    if !(condition) {
        ({
            log_472(
                facebook_yoga_LogLevel_Fatal,
                Ptr::<()>::from_string_literal(b"%s\n"),
                Box::new([((message).clone()).into()]),
            )
        });
        ({ fatalWithMessage_97((message).clone()) });
    }
}
pub fn assertFatalWithNode_214(
    mut node: Ptr<facebook_yoga_Node>,
    mut condition: (),
    mut message: Ptr<()>,
) {
    if !(condition) {
        ({
            log_473(
                (node).clone(),
                facebook_yoga_LogLevel_Fatal,
                Ptr::<()>::from_string_literal(b"%s\n"),
                Box::new([((message).clone()).into()]),
            )
        });
        ({ fatalWithMessage_97((message).clone()) });
    }
}
pub fn assertFatalWithConfig_190(
    mut config: Ptr<facebook_yoga_Config>,
    mut condition: (),
    mut message: Ptr<()>,
) {
    if !(condition) {
        ({
            log_474(
                (config).clone(),
                facebook_yoga_LogLevel_Fatal,
                Ptr::<()>::from_string_literal(b"%s\n"),
                Box::new([((message).clone()).into()]),
            )
        });
        ({ fatalWithMessage_97((message).clone()) });
    }
}
pub fn vlog_475(
    mut config: Ptr<facebook_yoga_Config>,
    mut node: Ptr<facebook_yoga_Node>,
    mut level: facebook_yoga_LogLevel,
    mut format: Ptr<()>,
    args: Ptr<VaArg>,
) {
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(args));
    if (config).is_null() {
        ({
            ({ getDefaultLogger_184() }).call(
                Ptr::<YGConfig>::null(),
                (node).clone(),
                ({ unscopedEnum_59(level) }),
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
pub fn log_472(mut level: facebook_yoga_LogLevel, format: Ptr<()>, __args: Box<[VaArg]>) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let format: Value<Ptr<()>> = Rc::new(RefCell::new(format));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    ({
        vlog_475(
            Ptr::<facebook_yoga_Config>::null(),
            Ptr::<facebook_yoga_Node>::null(),
            level,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn log_473(
    mut node: Ptr<facebook_yoga_Node>,
    mut level: facebook_yoga_LogLevel,
    format: Ptr<()>,
    __args: Box<[VaArg]>,
) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let format: Value<Ptr<()>> = Rc::new(RefCell::new(format));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    ({
        let _config: Ptr<facebook_yoga_Config> = if (node).is_null() {
            Ptr::<facebook_yoga_Config>::null()
        } else {
            ({ facebook_yoga_NodeImpl::getConfig(&node) })
        };
        let _node: Ptr<facebook_yoga_Node> = (node).clone();
        vlog_475(
            _config,
            _node,
            level,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn log_474(
    mut config: Ptr<facebook_yoga_Config>,
    mut level: facebook_yoga_LogLevel,
    format: Ptr<()>,
    __args: Box<[VaArg]>,
) {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let format: Value<Ptr<()>> = Rc::new(RefCell::new(format));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*args.borrow_mut()) = __args.as_pointer();
    ({
        vlog_475(
            (config).clone(),
            Ptr::<facebook_yoga_Node>::null(),
            level,
            (*format.borrow()).clone(),
            (*args.borrow()).clone(),
        )
    });
}
pub fn getDefaultLogger_184()
-> FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()> {
    return FnPtr::<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>::new(
        |mut _a0: Ptr<YGConfig>,
         mut _a1: Ptr<YGNode>,
         mut level: YGLogLevel,
         mut format: Ptr<()>,
         args: Ptr<VaArg>|
         -> () {
            {
                let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(args));
                'switch: {
                    match { (level as ()) } {
                        __v if __v == (YGLogLevel_YGLogLevelError as ())
                            || __v == (YGLogLevel_YGLogLevelFatal as ()) =>
                        {
                            return ({
                                vfprintf_476(unimplemented!(), format, (*args.borrow()).clone())
                            });
                        }
                        _ => {
                            return ({ vprintf_477(format, (*args.borrow()).clone()) });
                        }
                    }
                };
                panic!("ub: non-void function does not return a value")
            }
        },
    );
}
pub fn LayoutPassReasonToString_478(mut value: facebook_yoga_LayoutPassReason) -> Ptr<()> {
    'switch: {
        match { value } {
            __v if __v == facebook_yoga_LayoutPassReason_kInitial => {
                return Ptr::<()>::from_string_literal(b"initial");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kAbsLayout => {
                return Ptr::<()>::from_string_literal(b"abs_layout");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kStretch => {
                return Ptr::<()>::from_string_literal(b"stretch");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kMultilineStretch => {
                return Ptr::<()>::from_string_literal(b"multiline_stretch");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kFlexLayout => {
                return Ptr::<()>::from_string_literal(b"flex_layout");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kMeasureChild => {
                return Ptr::<()>::from_string_literal(b"measure");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kAbsMeasureChild => {
                return Ptr::<()>::from_string_literal(b"abs_measure");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kFlexMeasure => {
                return Ptr::<()>::from_string_literal(b"flex_measure");
            }
            __v if __v == facebook_yoga_LayoutPassReason_kGridLayout => {
                return Ptr::<()>::from_string_literal(b"grid_layout");
            }
            _ => {
                return Ptr::<()>::from_string_literal(b"unknown");
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
impl facebook_yoga_Node {
    pub fn new(subscriber: Ptr<()>) -> Self {
        Self {
            subscriber: unimplemented!(),
            next: Ptr::<facebook_yoga_Node>::null(),
        }
    }
    pub fn move_from(_a0: Ptr<facebook_yoga_Node>) -> Self {
        Self {
            subscriber: unimplemented!(),
            next: { (*_a0.upgrade().deref()).next.clone() },
        }
    }
}
thread_local!(
    pub static subscribers_479: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn push_480(mut newHead: Ptr<facebook_yoga_Node>) -> Ptr<facebook_yoga_Node> {
    let mut oldHead: Ptr<facebook_yoga_Node> = Ptr::<facebook_yoga_Node>::null();
    let mut __do_while = true;
    'loop_: while __do_while || (!(unimplemented!())) {
        __do_while = false;
        oldHead = unimplemented!();
        if !((newHead).is_null()) {
            field!(newHead, next).write((oldHead).clone());
        }
    }
    return oldHead;
}
impl facebook_yoga_Event {
    pub fn reset() {
        let mut head: Ptr<facebook_yoga_Node> = ({ push_480(Ptr::<facebook_yoga_Node>::null()) });
        'loop_: while !((head).is_null()) {
            let mut current: Ptr<facebook_yoga_Node> = (head).clone();
            head = { head.with(|__s| __s.next.clone()) };
            current.delete();
        }
    }
}
impl facebook_yoga_Event {
    pub fn subscribe(subscriber: Ptr<()>) {
        ({
            push_480(Ptr::alloc(facebook_yoga_Node::new({
                (subscriber).clone()
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
        let mut subscriber: Ptr<facebook_yoga_Node> = unimplemented!();
        'loop_: while !((subscriber).is_null()) {
            unimplemented!();
            subscriber = { subscriber.with(|__s| __s.next.clone()) };
        }
    }
}
pub fn inexactEquals_481(val1: Ptr<()>, val2: Ptr<()>) -> () {
    let mut areEqual: () = true;
    let mut i: () = 0_();
    'loop_: while (i < 4_()) && (areEqual) {
        areEqual = ({
            let _a: () = (unimplemented!().read());
            let _b: () = (unimplemented!().read());
            inexactEquals_105(_a, _b)
        });
        i.prefix_inc();
    }
    return areEqual;
}
pub fn inexactEquals_482(val1: Ptr<()>, val2: Ptr<()>) -> () {
    let mut areEqual: () = true;
    let mut i: () = 0_();
    'loop_: while (i < 2_()) && (areEqual) {
        areEqual = ({
            let _a: () = (unimplemented!().read());
            let _b: () = (unimplemented!().read());
            inexactEquals_105(_a, _b)
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
            context_: unimplemented!(),
            measureFunc_:
                FnPtr::<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>::null(),
            minContentMeasureFunc_: FnPtr::<
                fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
            >::null(),
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: FnPtr::<fn(Ptr<YGNode>, (), ()) -> ()>::null(),
            dirtiedFunc_: FnPtr::<fn(Ptr<YGNode>)>::null(),
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_(),
            contentsChildrenCount_: 0_(),
            owner_: Ptr::<facebook_yoga_Node>::null(),
            children_: unimplemented!(),
            config_: Ptr::<facebook_yoga_Config>::null(),
            processedDimensions_: unimplemented!(),
        }
    }
    pub fn new_1(mut config: Ptr<facebook_yoga_Config>) -> Self {
        let __this: Value<facebook_yoga_Node> = Rc::new(RefCell::new(Self {
            hasNewLayout_: true,
            isReferenceBaseline_: false,
            isDirty_: true,
            alwaysFormsContainingBlock_: false,
            nodeType_: facebook_yoga_NodeType_Default,
            context_: unimplemented!(),
            measureFunc_:
                FnPtr::<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>::null(),
            minContentMeasureFunc_: FnPtr::<
                fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
            >::null(),
            minContentWidth_: <facebook_yoga_FloatOptional>::default(),
            minContentHeight_: <facebook_yoga_FloatOptional>::default(),
            baselineFunc_: FnPtr::<fn(Ptr<YGNode>, (), ()) -> ()>::null(),
            dirtiedFunc_: FnPtr::<fn(Ptr<YGNode>)>::null(),
            style_: <facebook_yoga_Style>::default(),
            layout_: <facebook_yoga_LayoutResults>::default(),
            lineIndex_: 0_(),
            contentsChildrenCount_: 0_(),
            owner_: Ptr::<facebook_yoga_Node>::null(),
            children_: unimplemented!(),
            config_: (config).clone(),
            processedDimensions_: unimplemented!(),
        }));
        let this: Ptr<facebook_yoga_Node> = __this.as_pointer();
        ({
            assertFatal_202(
                !((config).is_null()),
                Ptr::<()>::from_string_literal(b"Attempting to construct Node with null config"),
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
            children_: unimplemented!(),
            config_: node.with(|__s| __s.config_.clone()),
            processedDimensions_: node.with(|__s| __s.processedDimensions_.clone()),
        }));
        let this: Ptr<facebook_yoga_Node> = __this.as_pointer();
        'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut c: Ptr<facebook_yoga_Node> = (unimplemented!().read());
            ({ facebook_yoga_NodeImpl::setOwner(&c, (this).clone()) });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {}
impl facebook_yoga_Node {}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_MeasureCallbackStart_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineEnd_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_NodeBaselineStart_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct facebook_yoga_Event_TypedData_facebook_yoga_Event_LayoutPassStart_;
pub trait facebook_yoga_CachedMeasurementImpl {
    fn operator_eq(&self, measurement: facebook_yoga_CachedMeasurement) -> ();
}
impl facebook_yoga_CachedMeasurementImpl for Ptr<facebook_yoga_CachedMeasurement> {
    fn operator_eq(&self, mut measurement: facebook_yoga_CachedMeasurement) -> () {
        let mut isEqual: () = ((*self).with(|__s| __s.widthSizingMode)
            == measurement.widthSizingMode)
            && ((*self).with(|__s| __s.heightSizingMode) == measurement.heightSizingMode);
        if (!({ isUndefined_99((*self).with(|__s| __s.availableWidth)) }))
            || (!({ isUndefined_99(measurement.availableWidth) }))
        {
            isEqual = {
                (isEqual) && ((*self).with(|__s| __s.availableWidth) == measurement.availableWidth)
            };
        }
        if (!({ isUndefined_99((*self).with(|__s| __s.availableHeight)) }))
            || (!({ isUndefined_99(measurement.availableHeight) }))
        {
            isEqual = {
                (isEqual)
                    && ((*self).with(|__s| __s.availableHeight) == measurement.availableHeight)
            };
        }
        if (!({ isUndefined_99((*self).with(|__s| __s.computedWidth)) }))
            || (!({ isUndefined_99(measurement.computedWidth) }))
        {
            isEqual = {
                (isEqual) && ((*self).with(|__s| __s.computedWidth) == measurement.computedWidth)
            };
        }
        if (!({ isUndefined_99((*self).with(|__s| __s.computedHeight)) }))
            || (!({ isUndefined_99(measurement.computedHeight) }))
        {
            isEqual = {
                (isEqual) && ((*self).with(|__s| __s.computedHeight) == measurement.computedHeight)
            };
        }
        if (!({ isUndefined_99((*self).with(|__s| __s.ownerWidth)) }))
            || (!({ isUndefined_99(measurement.ownerWidth) }))
        {
            isEqual =
                { (isEqual) && ((*self).with(|__s| __s.ownerWidth) == measurement.ownerWidth) };
        }
        if (!({ isUndefined_99((*self).with(|__s| __s.ownerHeight)) }))
            || (!({ isUndefined_99(measurement.ownerHeight) }))
        {
            isEqual =
                { (isEqual) && ((*self).with(|__s| __s.ownerHeight) == measurement.ownerHeight) };
        }
        return isEqual;
    }
}
pub trait facebook_yoga_ConfigImpl {
    fn setUseWebDefaults(&self, mut useWebDefaults: ()) {
        unimplemented!()
    }
    fn useWebDefaults(&self) -> () {
        unimplemented!()
    }
    fn setExperimentalFeatureEnabled(
        &self,
        mut feature: facebook_yoga_ExperimentalFeature,
        mut enabled: (),
    ) {
        unimplemented!()
    }
    fn isExperimentalFeatureEnabled(&self, mut feature: facebook_yoga_ExperimentalFeature) -> () {
        unimplemented!()
    }
    fn getEnabledExperiments(&self) -> () {
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
    fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> () {
        unimplemented!()
    }
    fn setPointScaleFactor(&self, mut pointScaleFactor: ()) {
        unimplemented!()
    }
    fn getPointScaleFactor(&self) -> () {
        unimplemented!()
    }
    fn setContext(&self, mut context: ()) {
        unimplemented!()
    }
    fn getContext(&self) -> () {
        unimplemented!()
    }
    fn getVersion(&self) -> () {
        unimplemented!()
    }
    fn setLogger(
        &self,
        mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>,
    ) {
        unimplemented!()
    }
    fn log(
        &self,
        mut node: Ptr<facebook_yoga_Node>,
        mut logLevel: facebook_yoga_LogLevel,
        mut format: Ptr<()>,
        args: Ptr<VaArg>,
    ) {
        unimplemented!()
    }
    fn setCloneNodeCallback(
        &self,
        mut cloneNode: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, ()) -> Ptr<YGNode>>,
    ) {
        unimplemented!()
    }
    fn cloneNode(
        &self,
        mut node: Ptr<YGNode>,
        mut owner: Ptr<YGNode>,
        mut childIndex: (),
    ) -> Ptr<YGNode> {
        unimplemented!()
    }
}
impl facebook_yoga_ConfigImpl for Ptr<facebook_yoga_Config> {
    fn setUseWebDefaults(&self, mut useWebDefaults: ()) {
        field!((*self), useWebDefaults_).write(useWebDefaults);
    }
    fn useWebDefaults(&self) -> () {
        return (*self).with(|__s| __s.useWebDefaults_);
    }
    fn setExperimentalFeatureEnabled(
        &self,
        mut feature: facebook_yoga_ExperimentalFeature,
        mut enabled: (),
    ) {
        if (({ facebook_yoga_ConfigImpl::isExperimentalFeatureEnabled(self, feature) }) != enabled)
        {
            unimplemented!();
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn isExperimentalFeatureEnabled(&self, mut feature: facebook_yoga_ExperimentalFeature) -> () {
        return unimplemented!();
    }
    fn getEnabledExperiments(&self) -> () {
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
                operator_bitor_assign_45(_a, errata)
            });
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn removeErrata(&self, mut errata: facebook_yoga_Errata) {
        if ({ facebook_yoga_ConfigImpl::hasErrata(self, errata) }) {
            ({
                let _a: Ptr<facebook_yoga_Errata> = field_ptr!((*self), errata_);
                operator_bitand_assign_46(
                    _a,
                    ({
                        let _a: facebook_yoga_Errata = errata;
                        operator_bitnot_41(_a)
                    }),
                )
            });
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn getErrata(&self) -> facebook_yoga_Errata {
        return (*self).with(|__s| __s.errata_);
    }
    fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> () {
        return (({
            let _a: facebook_yoga_Errata = (*self).with(|__s| __s.errata_);
            operator_bitand_43(_a, errata)
        }) != facebook_yoga_Errata_None);
    }
    fn setPointScaleFactor(&self, mut pointScaleFactor: ()) {
        if ((*self).with(|__s| __s.pointScaleFactor_) != pointScaleFactor) {
            field!((*self), pointScaleFactor_).write(pointScaleFactor);
            field!((*self), version_).with_mut(|__v| __v.postfix_inc());
        }
    }
    fn getPointScaleFactor(&self) -> () {
        return (*self).with(|__s| __s.pointScaleFactor_);
    }
    fn setContext(&self, mut context: ()) {
        field!((*self), context_).write((context).clone());
    }
    fn getContext(&self) -> () {
        return (*self).with(|__s| __s.context_.clone());
    }
    fn getVersion(&self) -> () {
        return (*self).with(|__s| __s.version_);
    }
    fn setLogger(
        &self,
        mut logger: FnPtr<fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> ()>,
    ) {
        field!((*self), logger_).write((logger).clone());
    }
    fn log(
        &self,
        mut node: Ptr<facebook_yoga_Node>,
        mut logLevel: facebook_yoga_LogLevel,
        mut format: Ptr<()>,
        args: Ptr<VaArg>,
    ) {
        let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(args));
        let mut logger: FnPtr<
            fn(Ptr<YGConfig>, Ptr<YGNode>, YGLogLevel, Ptr<()>, Ptr<VaArg>) -> (),
        > = if !(((*self).with(|__s| __s.logger_.clone())).is_null()) {
            (*self).with(|__s| __s.logger_.clone())
        } else {
            ({ getDefaultLogger_184() })
        };
        ({
            logger.call(
                (*self).clone(),
                (node).clone(),
                ({ unscopedEnum_59(logLevel) }),
                (format).clone(),
                (*args.borrow()).clone(),
            )
        });
    }
    fn setCloneNodeCallback(
        &self,
        mut cloneNode: FnPtr<fn(Ptr<YGNode>, Ptr<YGNode>, ()) -> Ptr<YGNode>>,
    ) {
        field!((*self), cloneNodeCallback_).write((cloneNode).clone());
    }
    fn cloneNode(
        &self,
        mut node: Ptr<YGNode>,
        mut owner: Ptr<YGNode>,
        mut childIndex: (),
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
            clone = ({ YGNodeClone_203((node).clone()) });
        }
        return clone;
    }
}
pub trait facebook_yoga_FloatOptionalImpl {
    fn unwrap(&self) -> ();
    fn unwrapOrDefault(&self, defaultValue: ()) -> ();
    fn isUndefined(&self) -> ();
    fn isDefined(&self) -> ();
}
impl facebook_yoga_FloatOptionalImpl for Ptr<facebook_yoga_FloatOptional> {
    fn unwrap(&self) -> () {
        return (*self).with(|__s| __s.value_);
    }
    fn unwrapOrDefault(&self, defaultValue: ()) -> () {
        let defaultValue: Value<()> = Rc::new(RefCell::new(defaultValue));
        return if ({ facebook_yoga_FloatOptionalImpl::isUndefined(self) }) {
            (*defaultValue.borrow())
        } else {
            (*self).with(|__s| __s.value_)
        };
    }
    fn isUndefined(&self) -> () {
        return ({ isUndefined_99((*self).with(|__s| __s.value_)) });
    }
    fn isDefined(&self) -> () {
        return ({ isDefined_101((*self).with(|__s| __s.value_)) });
    }
}
pub trait facebook_yoga_GridLineImpl {
    fn isAuto(&self) -> ();
    fn isInteger(&self) -> ();
    fn isSpan(&self) -> ();
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridLine>) -> ();
}
impl facebook_yoga_GridLineImpl for Ptr<facebook_yoga_GridLine> {
    fn isAuto(&self) -> () {
        return ((*self).with(|__s| __s.type_) == facebook_yoga_GridLineType_Auto);
    }
    fn isInteger(&self) -> () {
        return ((*self).with(|__s| __s.type_) == facebook_yoga_GridLineType_Integer);
    }
    fn isSpan(&self) -> () {
        return ((*self).with(|__s| __s.type_) == facebook_yoga_GridLineType_Span);
    }
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridLine>) -> () {
        return ({ (*self).with(|__s| __s.type_) } == { other.with(|__s| __s.type_) })
            && ({ (*self).with(|__s| __s.integer) } == { other.with(|__s| __s.integer) });
    }
}
pub trait facebook_yoga_GridStyleImpl {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyle>) -> ();
    fn move_assign(&self, _a0: Ptr<facebook_yoga_GridStyle>) -> Ptr<facebook_yoga_GridStyle>;
}
impl facebook_yoga_GridStyleImpl for Ptr<facebook_yoga_GridStyle> {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyle>) -> () {
        return (unimplemented!())
            && ((unimplemented!())
                && ((unimplemented!())
                    && ((unimplemented!())
                        && (({
                            let _other: Ptr<facebook_yoga_GridLine> =
                                field_ptr!(other, columnStart);
                            facebook_yoga_GridLineImpl::operator_eq(
                                &field_ptr!((*self), columnStart),
                                _other,
                            )
                        }) && (({
                            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, columnEnd);
                            facebook_yoga_GridLineImpl::operator_eq(
                                &field_ptr!((*self), columnEnd),
                                _other,
                            )
                        }) && (({
                            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, rowStart);
                            facebook_yoga_GridLineImpl::operator_eq(
                                &field_ptr!((*self), rowStart),
                                _other,
                            )
                        }) && ({
                            let _other: Ptr<facebook_yoga_GridLine> = field_ptr!(other, rowEnd);
                            facebook_yoga_GridLineImpl::operator_eq(
                                &field_ptr!((*self), rowEnd),
                                _other,
                            )
                        })))))));
    }
    fn move_assign(&self, _a0: Ptr<facebook_yoga_GridStyle>) -> Ptr<facebook_yoga_GridStyle> {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyleStorage>) -> ();
}
impl facebook_yoga_GridStyleStorageImpl for Ptr<facebook_yoga_GridStyleStorage> {
    fn move_assign(
        &self,
        _a0: Ptr<facebook_yoga_GridStyleStorage>,
    ) -> Ptr<facebook_yoga_GridStyleStorage> {
        unimplemented!();
        return (*self).clone();
    }
    fn copy_assign(
        &self,
        other: Ptr<facebook_yoga_GridStyleStorage>,
    ) -> Ptr<facebook_yoga_GridStyleStorage> {
        unimplemented!();
        return (*self).clone();
    }
    fn get(&self) -> Ptr<facebook_yoga_GridStyle> {
        return if unimplemented!() {
            ({ facebook_yoga_GridStyleStorage::defaults() })
        } else {
            (unimplemented!())
        };
    }
    fn ensure(&self) -> Ptr<facebook_yoga_GridStyle> {
        if unimplemented!() {
            unimplemented!();
        }
        return unimplemented!();
    }
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridStyleStorage>) -> () {
        return (unimplemented!())
            || ({
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
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridTrackSize>) -> ();
}
impl facebook_yoga_GridTrackSizeImpl for Ptr<facebook_yoga_GridTrackSize> {
    fn operator_eq(&self, other: Ptr<facebook_yoga_GridTrackSize>) -> () {
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
        }) && ({ (*self).with(|__s| __s.infinitelyGrowable) } == {
            other.with(|__s| __s.infinitelyGrowable)
        }))));
    }
}
pub trait facebook_yoga_LayoutResultsImpl {
    fn direction(&self) -> facebook_yoga_Direction;
    fn setDirection(&self, direction: facebook_yoga_Direction);
    fn hadOverflow(&self) -> ();
    fn setHadOverflow(&self, hadOverflow: ());
    fn dimension(&self, axis: facebook_yoga_Dimension) -> ();
    fn setDimension(&self, axis: facebook_yoga_Dimension, dimension: ());
    fn measuredDimension(&self, axis: facebook_yoga_Dimension) -> ();
    fn rawDimension(&self, axis: facebook_yoga_Dimension) -> ();
    fn setMeasuredDimension(&self, axis: facebook_yoga_Dimension, dimension: ());
    fn setRawDimension(&self, axis: facebook_yoga_Dimension, dimension: ());
    fn position(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> ();
    fn setPosition(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: ());
    fn margin(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> ();
    fn setMargin(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: ());
    fn border(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> ();
    fn setBorder(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: ());
    fn padding(&self, physicalEdge: facebook_yoga_PhysicalEdge) -> ();
    fn setPadding(&self, physicalEdge: facebook_yoga_PhysicalEdge, dimension: ());
    fn operator_eq(&self, mut layout: facebook_yoga_LayoutResults) -> () {
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
    fn hadOverflow(&self) -> () {
        return (*self).with(|__s| __s.hadOverflow_);
    }
    fn setHadOverflow(&self, mut hadOverflow: ()) {
        field!((*self), hadOverflow_).write(hadOverflow);
    }
    fn dimension(&self, mut axis: facebook_yoga_Dimension) -> () {
        return (unimplemented!().read());
    }
    fn setDimension(&self, mut axis: facebook_yoga_Dimension, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn measuredDimension(&self, mut axis: facebook_yoga_Dimension) -> () {
        return (unimplemented!().read());
    }
    fn rawDimension(&self, mut axis: facebook_yoga_Dimension) -> () {
        return (unimplemented!().read());
    }
    fn setMeasuredDimension(&self, mut axis: facebook_yoga_Dimension, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn setRawDimension(&self, mut axis: facebook_yoga_Dimension, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn position(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (unimplemented!().read());
    }
    fn setPosition(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn margin(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (unimplemented!().read());
    }
    fn setMargin(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn border(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (unimplemented!().read());
    }
    fn setBorder(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn padding(&self, mut physicalEdge: facebook_yoga_PhysicalEdge) -> () {
        return (unimplemented!().read());
    }
    fn setPadding(&self, mut physicalEdge: facebook_yoga_PhysicalEdge, mut dimension: ()) {
        unimplemented!().write(dimension);
    }
    fn operator_eq(&self, layout: facebook_yoga_LayoutResults) -> () {
        let layout: Value<facebook_yoga_LayoutResults> = Rc::new(RefCell::new(layout));
        let mut isEqual: () = ((((((((((({
            let _val1: Ptr<()> = field_ptr!((*self), position_);
            let _val2: Ptr<()> = field_ptr!(layout.as_pointer(), position_);
            inexactEquals_481(_val1, _val2)
        }) && ({
            let _val1: Ptr<()> = field_ptr!((*self), dimensions_);
            let _val2: Ptr<()> = field_ptr!(layout.as_pointer(), dimensions_);
            inexactEquals_482(_val1, _val2)
        })) && ({
            let _val1: Ptr<()> = field_ptr!((*self), margin_);
            let _val2: Ptr<()> = field_ptr!(layout.as_pointer(), margin_);
            inexactEquals_481(_val1, _val2)
        })) && ({
            let _val1: Ptr<()> = field_ptr!((*self), border_);
            let _val2: Ptr<()> = field_ptr!(layout.as_pointer(), border_);
            inexactEquals_481(_val1, _val2)
        })) && ({
            let _val1: Ptr<()> = field_ptr!((*self), padding_);
            let _val2: Ptr<()> = field_ptr!(layout.as_pointer(), padding_);
            inexactEquals_481(_val1, _val2)
        })) && (({
            facebook_yoga_LayoutResultsImpl::direction(self)
        }) == ({
            facebook_yoga_LayoutResultsImpl::direction(&layout.as_pointer())
        }))) && (({
            facebook_yoga_LayoutResultsImpl::hadOverflow(self)
        }) == ({
            facebook_yoga_LayoutResultsImpl::hadOverflow(&layout.as_pointer())
        }))) && ((*self).with(|__s| __s.lastOwnerDirection) == {
            (*layout.borrow()).lastOwnerDirection
        })) && ((*self).with(|__s| __s.configVersion) == {
            (*layout.borrow()).configVersion
        })) && ((*self).with(|__s| __s.nextCachedMeasurementsIndex) == {
            (*layout.borrow()).nextCachedMeasurementsIndex
        })) && ({
            let _measurement: facebook_yoga_CachedMeasurement =
                { (*layout.borrow()).cachedLayout.clone() };
            facebook_yoga_CachedMeasurementImpl::operator_eq(
                &field_ptr!((*self), cachedLayout),
                _measurement,
            )
        })) && ({
            let _lhs: facebook_yoga_FloatOptional =
                (*self).with(|__s| __s.computedFlexBasis.clone());
            let _rhs: facebook_yoga_FloatOptional =
                { (*layout.borrow()).computedFlexBasis.clone() };
            operator_eq_108(_lhs, _rhs)
        });
        let mut i: () = 0_();
        'loop_: while (i < MaxCachedMeasurements_117.with(|rc| *rc.borrow())) && (isEqual) {
            let __rhs = (isEqual)
                && ({
                    let _measurement: facebook_yoga_CachedMeasurement =
                        (*unimplemented!().upgrade().deref()).clone();
                    facebook_yoga_CachedMeasurementImpl::operator_eq(
                        &unimplemented!(),
                        _measurement,
                    )
                });
            isEqual = __rhs;
            i.prefix_inc();
        }
        if (!({ isUndefined_99((unimplemented!().read())) }))
            || (!({ isUndefined_99((unimplemented!().read())) }))
        {
            let __rhs = (isEqual) && ((unimplemented!().read()) == (unimplemented!().read()));
            isEqual = __rhs;
        }
        if (!({ isUndefined_99((unimplemented!().read())) }))
            || (!({ isUndefined_99((unimplemented!().read())) }))
        {
            let __rhs = (isEqual) && ((unimplemented!().read()) == (unimplemented!().read()));
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
            > 0_())
        {
            let result: Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator> =
                Rc::new(RefCell::new(
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator::new_1(
                        { (*self).with(|__s| __s.node_.clone()) },
                        { 0_() },
                    ),
                ));
            if (({
                facebook_yoga_StyleImpl::display(
                    &({
                        facebook_yoga_NodeImpl::style_24(
                            &({
                                facebook_yoga_NodeImpl::getChild(
                                    &(*self).with(|__s| __s.node_.clone()),
                                    0_(),
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
        _a0: (),
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
        mut _a0: (),
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
        if ({ ((*self).with(|__s| __s.childIndex_)).wrapping_add(1_()) } >= {
            ({ facebook_yoga_NodeImpl::getChildCount(&(*self).with(|__s| __s.node_.clone())) })
        }) {
            if unimplemented!() {
                {
                    ({
                        let _arg0 : Value<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator > = Rc::new(RefCell::new(<facebook_yoga_LayoutableChildren_facebook_yoga_Node__Iterator  >::default() ));
                        facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: move_assign ( &(* self )   , _arg0.as_pointer() , )
                    });
                };
            } else {
                let back: Ptr<()> = unimplemented!();
                field!((*self), node_).write(back.with(|__s| __s.first.clone()));
                field!((*self), childIndex_).write(back.with(|__s| __s.second));
                unimplemented!();
                ({ facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl::next(self) });
            }
        } else {
            field!((*self), childIndex_).with_mut(|__v| __v.prefix_inc());
            if (({
                facebook_yoga_StyleImpl::display(
                    &({
                        facebook_yoga_NodeImpl::style_24(
                            &({
                                let _index: () = (*self).with(|__s| __s.childIndex_);
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
            let _index: () = (*self).with(|__s| __s.childIndex_);
            facebook_yoga_NodeImpl::getChild(&(*self).with(|__s| __s.node_.clone()), _index)
        });
        'loop_: while (({
            facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&currentNode) }))
        }) == facebook_yoga_Display_Contents)
            && (({ facebook_yoga_NodeImpl::getChildCount(&currentNode) }) > 0_())
        {
            unimplemented!();
            field!((*self), node_).write((currentNode).clone());
            field!((*self), childIndex_).write(0_());
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
        unimplemented!();
        return (*self).clone();
    }
}
pub trait facebook_yoga_NodeImpl {
    fn getContext(&self) -> ();
    fn alwaysFormsContainingBlock(&self) -> ();
    fn getHasNewLayout(&self) -> ();
    fn getNodeType(&self) -> facebook_yoga_NodeType;
    fn hasMeasureFunc(&self) -> ();
    fn measure(
        &self,
        mut availableWidth: (),
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: (),
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        unimplemented!()
    }
    fn hasMinContentMeasureFunc(&self) -> ();
    fn measureMinContent(
        &self,
        mut availableWidth: (),
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: (),
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        unimplemented!()
    }
    fn getMinContentWidth(&self) -> facebook_yoga_FloatOptional;
    fn getMinContentHeight(&self) -> facebook_yoga_FloatOptional;
    fn hasBaselineFunc(&self) -> ();
    fn baseline(&self, mut width: (), mut height: ()) -> () {
        unimplemented!()
    }
    fn dimensionWithMargin(&self, mut axis: facebook_yoga_FlexDirection, mut widthSize: ()) -> () {
        unimplemented!()
    }
    fn isLayoutDimensionDefined(&self, mut axis: facebook_yoga_FlexDirection) -> () {
        unimplemented!()
    }
    fn hasDefiniteLength(&self, dimension: facebook_yoga_Dimension, ownerSize: ()) -> ();
    fn hasErrata(&self, errata: facebook_yoga_Errata) -> ();
    fn hasContentsChildren(&self) -> ();
    fn getDirtiedFunc(&self) -> FnPtr<fn(Ptr<YGNode>)>;
    fn style_24(&self) -> Ptr<facebook_yoga_Style>;
    fn style_25(&self) -> Ptr<facebook_yoga_Style>;
    fn getLayout_26(&self) -> Ptr<facebook_yoga_LayoutResults>;
    fn getLayout_27(&self) -> Ptr<facebook_yoga_LayoutResults>;
    fn getLineIndex(&self) -> ();
    fn isReferenceBaseline(&self) -> ();
    fn getOwner(&self) -> Ptr<facebook_yoga_Node>;
    fn getChildren(&self) -> Ptr<()>;
    fn getChild(&self, index: ()) -> Ptr<facebook_yoga_Node>;
    fn getChildCount(&self) -> ();
    fn getLayoutChildren(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node_;
    fn getLayoutChildCount(&self) -> ();
    fn getConfig(&self) -> Ptr<facebook_yoga_Config>;
    fn isDirty(&self) -> ();
    fn getProcessedDimension(
        &self,
        dimension: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength;
    fn getResolvedDimension(
        &self,
        direction: facebook_yoga_Direction,
        dimension: facebook_yoga_Dimension,
        referenceLength: (),
        ownerWidth: (),
    ) -> facebook_yoga_FloatOptional;
    fn setContext(&self, context: ());
    fn setAlwaysFormsContainingBlock(&self, alwaysFormsContainingBlock: ());
    fn setHasNewLayout(&self, hasNewLayout: ());
    fn setNodeType(&self, nodeType: facebook_yoga_NodeType);
    fn setMeasureFunc(
        &self,
        mut measureFunc: FnPtr<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
    ) {
        unimplemented!()
    }
    fn setMinContentMeasureFunc(
        &self,
        minContentMeasureFunc: FnPtr<
            fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
        >,
    );
    fn setMinContentWidth(&self, minContentWidth: facebook_yoga_FloatOptional);
    fn setMinContentHeight(&self, minContentHeight: facebook_yoga_FloatOptional);
    fn setBaselineFunc(&self, baseLineFunc: FnPtr<fn(Ptr<YGNode>, (), ()) -> ()>);
    fn setDirtiedFunc(&self, dirtiedFunc: FnPtr<fn(Ptr<YGNode>)>);
    fn setStyle(&self, style: Ptr<facebook_yoga_Style>);
    fn setLayout(&self, layout: Ptr<facebook_yoga_LayoutResults>);
    fn setLineIndex(&self, lineIndex: ());
    fn setIsReferenceBaseline(&self, isReferenceBaseline: ());
    fn setOwner(&self, owner: Ptr<facebook_yoga_Node>);
    fn setConfig(&self, mut config: Ptr<facebook_yoga_Config>) {
        unimplemented!()
    }
    fn setDirty(&self, mut isDirty: ()) {
        unimplemented!()
    }
    fn setChildren(&self, children: Ptr<()>) {
        unimplemented!()
    }
    fn setLayoutLastOwnerDirection(&self, mut direction: facebook_yoga_Direction) {
        unimplemented!()
    }
    fn setLayoutComputedFlexBasis(&self, mut computedFlexBasis: facebook_yoga_FloatOptional) {
        unimplemented!()
    }
    fn setLayoutComputedFlexBasisGeneration(&self, mut computedFlexBasisGeneration: ()) {
        unimplemented!()
    }
    fn setLayoutMeasuredDimension(
        &self,
        mut measuredDimension: (),
        mut dimension: facebook_yoga_Dimension,
    ) {
        unimplemented!()
    }
    fn setLayoutHadOverflow(&self, mut hadOverflow: ()) {
        unimplemented!()
    }
    fn setLayoutDimension(&self, mut lengthValue: (), mut dimension: facebook_yoga_Dimension) {
        unimplemented!()
    }
    fn setLayoutDirection(&self, mut direction: facebook_yoga_Direction) {
        unimplemented!()
    }
    fn setLayoutMargin(&self, mut margin: (), mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setLayoutBorder(&self, mut border: (), mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setLayoutPadding(&self, mut padding: (), mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setLayoutPosition(&self, mut position: (), mut edge: facebook_yoga_PhysicalEdge) {
        unimplemented!()
    }
    fn setPosition(
        &self,
        mut direction: facebook_yoga_Direction,
        mut ownerWidth: (),
        mut ownerHeight: (),
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
        mut referenceLength: (),
        mut ownerWidth: (),
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
    fn replaceChild_76(&self, mut child: Ptr<facebook_yoga_Node>, mut index: ()) {
        unimplemented!()
    }
    fn insertChild(&self, mut child: Ptr<facebook_yoga_Node>, mut index: ()) {
        unimplemented!()
    }
    fn removeChild_78(&self, mut child: Ptr<facebook_yoga_Node>) -> () {
        unimplemented!()
    }
    fn removeChild_79(&self, mut index: ()) {
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
    fn resolveFlexGrow(&self) -> () {
        unimplemented!()
    }
    fn resolveFlexShrink(&self) -> () {
        unimplemented!()
    }
    fn isNodeFlexible(&self) -> () {
        unimplemented!()
    }
    fn reset(&self) {
        unimplemented!()
    }
    fn relativePosition(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut axisSize: (),
    ) -> () {
        unimplemented!()
    }
    fn useWebDefaults(&self);
}
impl facebook_yoga_NodeImpl for Ptr<facebook_yoga_Node> {
    fn getContext(&self) -> () {
        return (*self).with(|__s| __s.context_.clone());
    }
    fn alwaysFormsContainingBlock(&self) -> () {
        return (*self).with(|__s| __s.alwaysFormsContainingBlock_);
    }
    fn getHasNewLayout(&self) -> () {
        return (*self).with(|__s| __s.hasNewLayout_);
    }
    fn getNodeType(&self) -> facebook_yoga_NodeType {
        return (*self).with(|__s| __s.nodeType_);
    }
    fn hasMeasureFunc(&self) -> () {
        return !(((*self).with(|__s| __s.measureFunc_.clone())).is_null());
    }
    fn hasMinContentMeasureFunc(&self) -> () {
        return !(((*self).with(|__s| __s.minContentMeasureFunc_.clone())).is_null());
    }
    fn getMinContentWidth(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.minContentWidth_.clone());
    }
    fn getMinContentHeight(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.minContentHeight_.clone());
    }
    fn hasBaselineFunc(&self) -> () {
        return !(((*self).with(|__s| __s.baselineFunc_.clone())).is_null());
    }
    fn hasDefiniteLength(&self, mut dimension: facebook_yoga_Dimension, mut ownerSize: ()) -> () {
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
    fn hasErrata(&self, mut errata: facebook_yoga_Errata) -> () {
        return ({
            facebook_yoga_ConfigImpl::hasErrata(&(*self).with(|__s| __s.config_.clone()), errata)
        });
    }
    fn hasContentsChildren(&self) -> () {
        return ((*self).with(|__s| __s.contentsChildrenCount_) != 0_());
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
    fn getLineIndex(&self) -> () {
        return (*self).with(|__s| __s.lineIndex_);
    }
    fn isReferenceBaseline(&self) -> () {
        return (*self).with(|__s| __s.isReferenceBaseline_);
    }
    fn getOwner(&self) -> Ptr<facebook_yoga_Node> {
        return (*self).with(|__s| __s.owner_.clone());
    }
    fn getChildren(&self) -> Ptr<()> {
        return field_ptr!((*self), children_);
    }
    fn getChild(&self, mut index: ()) -> Ptr<facebook_yoga_Node> {
        return (unimplemented!().read());
    }
    fn getChildCount(&self) -> () {
        return unimplemented!();
    }
    fn getLayoutChildren(&self) -> facebook_yoga_LayoutableChildren_facebook_yoga_Node_ {
        return facebook_yoga_LayoutableChildren_facebook_yoga_Node_::new({ (*self).clone() });
    }
    fn getLayoutChildCount(&self) -> () {
        if ((*self).with(|__s| __s.contentsChildrenCount_) == 0_()) {
            return unimplemented!();
        } else {
            let mut count: () = 0_();
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
                operator_ne_70(_a, _b.as_pointer())
            }) {
                count.postfix_inc();
                ({
                    facebook_yoga_LayoutableChildren_facebook_yoga_Node__IteratorImpl :: operator_post_inc_5 ( &iter .as_pointer()  , 0_()  , )
                });
            }
            return count;
        }
        panic!("ub: non-void function does not return a value")
    }
    fn getConfig(&self) -> Ptr<facebook_yoga_Config> {
        return (*self).with(|__s| __s.config_.clone());
    }
    fn isDirty(&self) -> () {
        return (*self).with(|__s| __s.isDirty_);
    }
    fn getProcessedDimension(
        &self,
        mut dimension: facebook_yoga_Dimension,
    ) -> facebook_yoga_StyleSizeLength {
        return (*unimplemented!().upgrade().deref()).clone();
    }
    fn getResolvedDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut referenceLength: (),
        mut ownerWidth: (),
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
            operator_add_111(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    fn setContext(&self, mut context: ()) {
        field!((*self), context_).write((context).clone());
    }
    fn setAlwaysFormsContainingBlock(&self, mut alwaysFormsContainingBlock: ()) {
        field!((*self), alwaysFormsContainingBlock_).write(alwaysFormsContainingBlock);
    }
    fn setHasNewLayout(&self, mut hasNewLayout: ()) {
        field!((*self), hasNewLayout_).write(hasNewLayout);
    }
    fn setNodeType(&self, mut nodeType: facebook_yoga_NodeType) {
        field!((*self), nodeType_).write(nodeType);
    }
    fn setMinContentMeasureFunc(
        &self,
        mut minContentMeasureFunc: FnPtr<
            fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize,
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
    fn setBaselineFunc(&self, mut baseLineFunc: FnPtr<fn(Ptr<YGNode>, (), ()) -> ()>) {
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
    fn setLineIndex(&self, mut lineIndex: ()) {
        field!((*self), lineIndex_).write(lineIndex);
    }
    fn setIsReferenceBaseline(&self, mut isReferenceBaseline: ()) {
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
    fn move_assign(&self, _a0: Ptr<facebook_yoga_Node>) -> Ptr<facebook_yoga_Node> {
        unimplemented!();
        field!((*self), next).write({ { (*_a0.upgrade().deref()).next.clone() } });
        return (*self).clone();
    }
    fn measure(
        &self,
        mut availableWidth: (),
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: (),
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let mut size: YGSize = ({
            let _arg0: Ptr<YGNode> = (*self).clone();
            let _arg2: YGMeasureMode = ({ unscopedEnum_88(widthMode) });
            let _arg4: YGMeasureMode = ({ unscopedEnum_88(heightMode) });
            (*self).with(|__s| __s.measureFunc_.clone()).call(
                _arg0,
                availableWidth,
                _arg2,
                availableHeight,
                _arg4,
            )
        });
        if ((({ isUndefined_99(size.height) }) || (size.height < 0_()))
            || ({ isUndefined_99(size.width) }))
            || (size.width < 0_())
        {
            ({
                log_473 ( ((* self ) ).clone() , facebook_yoga_LogLevel_Warn  , Ptr::<() >::from_string_literal(b"Measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" )  , Box::new ( [ ( size . width  ) .into() , ( size . height  ) .into() , ] ) )
            });
            return YGSize {
                width: ({ maxOrDefined_104(0_f32, size.width) }),
                height: ({ maxOrDefined_104(0_f32, size.height) }),
            };
        }
        return size;
    }
    fn measureMinContent(
        &self,
        mut availableWidth: (),
        mut widthMode: facebook_yoga_MeasureMode,
        mut availableHeight: (),
        mut heightMode: facebook_yoga_MeasureMode,
    ) -> YGSize {
        let mut size: YGSize = ({
            let _arg0: Ptr<YGNode> = (*self).clone();
            let _arg2: YGMeasureMode = ({ unscopedEnum_88(widthMode) });
            let _arg4: YGMeasureMode = ({ unscopedEnum_88(heightMode) });
            (*self).with(|__s| __s.minContentMeasureFunc_.clone()).call(
                _arg0,
                availableWidth,
                _arg2,
                availableHeight,
                _arg4,
            )
        });
        if ((({ isUndefined_99(size.height) }) || (size.height < 0_()))
            || ({ isUndefined_99(size.width) }))
            || (size.width < 0_())
        {
            ({
                log_473 ( ((* self ) ).clone() , facebook_yoga_LogLevel_Warn  , Ptr::<() >::from_string_literal(b"Min-content measure function returned an invalid dimension to Yoga: [width=%f, height=%f]" )  , Box::new ( [ ( size . width  ) .into() , ( size . height  ) .into() , ] ) )
            });
            let __rhs = YGSize {
                width: ({ maxOrDefined_104(0_f32, size.width) }),
                height: ({ maxOrDefined_104(0_f32, size.height) }),
            };
            size = __rhs;
        }
        return size;
    }
    fn baseline(&self, mut width: (), mut height: ()) -> () {
        return ({
            let _arg0: Ptr<YGNode> = (*self).clone();
            (*self)
                .with(|__s| __s.baselineFunc_.clone())
                .call(_arg0, width, height)
        });
    }
    fn dimensionWithMargin(&self, mut axis: facebook_yoga_FlexDirection, mut widthSize: ()) -> () {
        return (({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(self) }),
                ({ dimension_131(axis) }),
            )
        }) + ({
            facebook_yoga_StyleImpl::computeMarginForAxis(
                &field_ptr!((*self), style_),
                axis,
                widthSize,
            )
        }));
    }
    fn isLayoutDimensionDefined(&self, mut axis: facebook_yoga_FlexDirection) -> () {
        let mut value: () = ({
            facebook_yoga_LayoutResultsImpl::measuredDimension(
                &({ facebook_yoga_NodeImpl::getLayout_26(self) }),
                ({ dimension_131(axis) }),
            )
        });
        return ({ isDefined_101(value) }) && (value >= 0_f32);
    }
    fn setMeasureFunc(
        &self,
        mut measureFunc: FnPtr<fn(Ptr<YGNode>, (), YGMeasureMode, (), YGMeasureMode) -> YGSize>,
    ) {
        if (measureFunc).is_null() {
            ({ facebook_yoga_NodeImpl::setNodeType(self, facebook_yoga_NodeType_Default) });
        } else {
            ({
                let _node: Ptr<facebook_yoga_Node> = (*self).clone();
                let _condition: () = unimplemented!();
                assertFatalWithNode_214 ( _node , _condition , Ptr::<() >::from_string_literal(b"Cannot set measure function: Nodes with measure functions cannot have children." )  , )
            });
            ({ facebook_yoga_NodeImpl::setNodeType(self, facebook_yoga_NodeType_Text) });
        }
        field!((*self), measureFunc_).write((measureFunc).clone());
    }
    fn replaceChild_76(&self, mut child: Ptr<facebook_yoga_Node>, mut index: ()) {
        let mut previousChild: () = (unimplemented!().read());
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
        unimplemented!().write((child).clone());
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
        unimplemented!();
    }
    fn insertChild(&self, mut child: Ptr<facebook_yoga_Node>, mut index: ()) {
        if (({ facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) })) })
            == facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_inc());
        }
        unimplemented!();
    }
    fn setConfig(&self, mut config: Ptr<facebook_yoga_Config>) {
        ({
            assertFatal_202(
                !((config).is_null()),
                Ptr::<()>::from_string_literal(b"Attempting to set a null config on a Node"),
            )
        });
        ({
            let _config: Ptr<facebook_yoga_Config> = (config).clone();
            let _condition: () = ({ ({ facebook_yoga_ConfigImpl::useWebDefaults(&config) }) } == {
                ({
                    facebook_yoga_ConfigImpl::useWebDefaults(
                        &(*self).with(|__s| __s.config_.clone()),
                    )
                })
            });
            assertFatalWithConfig_190(
                _config,
                _condition,
                Ptr::<()>::from_string_literal(
                    b"UseWebDefaults may not be changed after constructing a Node",
                ),
            )
        });
        if ({
            let _oldConfig: Ptr<facebook_yoga_Config> = (*self).with(|__s| __s.config_.clone());
            let _newConfig: Ptr<facebook_yoga_Config> = (config).clone();
            configUpdateInvalidatesLayout_471(_oldConfig, _newConfig)
        }) {
            ({ facebook_yoga_NodeImpl::markDirtyAndPropagate(self) });
            field!(field!((*self), layout_), configVersion).write(0_());
        } else {
            field!(field!((*self), layout_), configVersion)
                .write(({ facebook_yoga_ConfigImpl::getVersion(&config) }));
        }
        field!((*self), config_).write((config).clone());
    }
    fn setDirty(&self, mut isDirty: ()) {
        if (isDirty == (*self).with(|__s| __s.isDirty_)) {
            return;
        }
        field!((*self), isDirty_).write(isDirty);
        if (isDirty) && (!(((*self).with(|__s| __s.dirtiedFunc_.clone())).is_null())) {
            ({
                let _arg0: Ptr<YGNode> = (*self).clone();
                (*self).with(|__s| __s.dirtiedFunc_.clone()).call(_arg0)
            });
        }
    }
    fn setChildren(&self, children: Ptr<()>) {
        unimplemented!();
        field!((*self), contentsChildrenCount_).write(0_());
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let child: Ptr<Ptr<facebook_yoga_Node>> = unimplemented!();
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
    fn removeChild_78(&self, mut child: Ptr<facebook_yoga_Node>) -> () {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            if (({
                facebook_yoga_StyleImpl::display(&({ facebook_yoga_NodeImpl::style_24(&child) }))
            }) == facebook_yoga_Display_Contents)
            {
                field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_dec());
            }
            unimplemented!();
            return true;
        }
        return false;
    }
    fn removeChild_79(&self, mut index: ()) {
        if (({
            facebook_yoga_StyleImpl::display(
                &({ facebook_yoga_NodeImpl::style_24(&(unimplemented!().read())) }),
            )
        }) == facebook_yoga_Display_Contents)
        {
            field!((*self), contentsChildrenCount_).with_mut(|__v| __v.postfix_dec());
        }
        unimplemented!();
    }
    fn setLayoutDirection(&self, mut direction: facebook_yoga_Direction) {
        ({
            facebook_yoga_LayoutResultsImpl::setDirection(&field_ptr!((*self), layout_), direction)
        });
    }
    fn setLayoutMargin(&self, mut margin: (), mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setMargin(&field_ptr!((*self), layout_), edge, margin)
        });
    }
    fn setLayoutBorder(&self, mut border: (), mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setBorder(&field_ptr!((*self), layout_), edge, border)
        });
    }
    fn setLayoutPadding(&self, mut padding: (), mut edge: facebook_yoga_PhysicalEdge) {
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
    fn setLayoutPosition(&self, mut position: (), mut edge: facebook_yoga_PhysicalEdge) {
        ({
            facebook_yoga_LayoutResultsImpl::setPosition(
                &field_ptr!((*self), layout_),
                edge,
                position,
            )
        });
    }
    fn setLayoutComputedFlexBasisGeneration(&self, mut computedFlexBasisGeneration: ()) {
        field!(field!((*self), layout_), computedFlexBasisGeneration)
            .write(computedFlexBasisGeneration);
    }
    fn setLayoutMeasuredDimension(
        &self,
        mut measuredDimension: (),
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
    fn setLayoutHadOverflow(&self, mut hadOverflow: ()) {
        ({
            facebook_yoga_LayoutResultsImpl::setHadOverflow(
                &field_ptr!((*self), layout_),
                hadOverflow,
            )
        });
    }
    fn setLayoutDimension(&self, mut lengthValue: (), mut dimension: facebook_yoga_Dimension) {
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
        mut axisSize: (),
    ) -> () {
        if (({ facebook_yoga_StyleImpl::positionType(&field_ptr!((*self), style_)) })
            == facebook_yoga_PositionType_Static)
        {
            return 0_();
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
        return (-1_()
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
        mut ownerWidth: (),
        mut ownerHeight: (),
    ) {
        let mut directionRespectingRoot: facebook_yoga_Direction =
            if !(((*self).with(|__s| __s.owner_.clone())).is_null()) {
                direction
            } else {
                facebook_yoga_Direction_LTR
            };
        let mut mainAxis: facebook_yoga_FlexDirection = ({
            resolveDirection_125(
                ({ facebook_yoga_StyleImpl::flexDirection(&field_ptr!((*self), style_)) }),
                directionRespectingRoot,
            )
        });
        let mut crossAxis: facebook_yoga_FlexDirection =
            ({ resolveCrossDirection_126(mainAxis, directionRespectingRoot) });
        let mut relativePositionMain: () = ({
            let _axis: facebook_yoga_FlexDirection = mainAxis;
            let _axisSize: () = if ({ isRow_123(mainAxis) }) {
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
        let mut relativePositionCross: () = ({
            facebook_yoga_NodeImpl::relativePosition(
                self,
                crossAxis,
                directionRespectingRoot,
                if ({ isRow_123(mainAxis) }) {
                    ownerHeight
                } else {
                    ownerWidth
                },
            )
        });
        let mut mainAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineStartEdge_129(mainAxis, direction) });
        let mut mainAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineEndEdge_130(mainAxis, direction) });
        let mut crossAxisLeadingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineStartEdge_129(crossAxis, direction) });
        let mut crossAxisTrailingEdge: facebook_yoga_PhysicalEdge =
            ({ inlineEndEdge_130(crossAxis, direction) });
        ({
            let _position: () = (({
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
            let _position: () = (({
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
            let _position: () = (({
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
            let _position: () = (({
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
                ({ facebook_yoga_StyleSizeLength::points(0_()) })
            };
        }
        return ({ facebook_yoga_StyleSizeLength::ofAuto() });
    }
    fn resolveFlexBasis(
        &self,
        mut direction: facebook_yoga_Direction,
        mut flexDirection: facebook_yoga_FlexDirection,
        mut referenceLength: (),
        mut ownerWidth: (),
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
        let mut dim: facebook_yoga_Dimension = ({ dimension_131(flexDirection) });
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
            operator_add_111(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    fn processDimensions(&self) {
        'loop_: for dim in IterRange::new(unimplemented!(), unimplemented!()) {
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
                inexactEquals_172(_a.as_pointer(), _b.as_pointer())
            }) {
                let __rhs =
                    ({ facebook_yoga_StyleImpl::maxDimension(&field_ptr!((*self), style_), dim) });
                unimplemented!().write(__rhs);
            } else {
                let __rhs =
                    ({ facebook_yoga_StyleImpl::dimension(&field_ptr!((*self), style_), dim) });
                unimplemented!().write(__rhs);
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
        unimplemented!();
        unimplemented!();
    }
    fn cloneChildrenIfNeeded(&self) {
        let mut i: () = 0_();
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let child: Ptr<Ptr<facebook_yoga_Node>> = unimplemented!();
            if (({ facebook_yoga_NodeImpl::getOwner(&(child.read())) }) != (*self)) {
                let __rhs = ({
                    resolveRef_181(
                        ({
                            let _owner: Ptr<YGNode> = (*self).clone();
                            facebook_yoga_ConfigImpl::cloneNode(
                                &(*self).with(|__s| __s.config_.clone()),
                                (child.read()),
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
            i = { (i).wrapping_add(1_()) };
        }
    }
    fn cloneContentsChildrenIfNeeded(&self) {
        let mut i: () = 0_();
        'loop_: for child in IterRange::new(unimplemented!(), unimplemented!()) {
            let child: Ptr<Ptr<facebook_yoga_Node>> = unimplemented!();
            if (({
                facebook_yoga_StyleImpl::display(
                    &({ facebook_yoga_NodeImpl::style_24(&(child.read())) }),
                )
            }) == facebook_yoga_Display_Contents)
                && (({ facebook_yoga_NodeImpl::getOwner(&(child.read())) }) != (*self))
            {
                let __rhs = ({
                    resolveRef_181(
                        ({
                            let _owner: Ptr<YGNode> = (*self).clone();
                            facebook_yoga_ConfigImpl::cloneNode(
                                &(*self).with(|__s| __s.config_.clone()),
                                (child.read()),
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
            i = { (i).wrapping_add(1_()) };
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
    fn resolveFlexGrow(&self) -> () {
        if ((*self).with(|__s| __s.owner_.clone())).is_null() {
            return 0_f64;
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
        return DefaultFlexGrow_178.with(|rc| *rc.borrow());
    }
    fn resolveFlexShrink(&self) -> () {
        if ((*self).with(|__s| __s.owner_.clone())).is_null() {
            return 0_f64;
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
            WebDefaultFlexShrink_180.with(|rc| *rc.borrow())
        } else {
            DefaultFlexShrink_179.with(|rc| *rc.borrow())
        };
    }
    fn isNodeFlexible(&self) -> () {
        return ((({ facebook_yoga_StyleImpl::positionType(&field_ptr!((*self), style_)) })
            != facebook_yoga_PositionType_Absolute)
            && ((({ facebook_yoga_NodeImpl::resolveFlexGrow(self) }) != 0_())
                || (({ facebook_yoga_NodeImpl::resolveFlexShrink(self) }) != 0_())));
    }
    fn reset(&self) {
        ({
            let _node: Ptr<facebook_yoga_Node> = (*self).clone();
            let _condition: () = unimplemented!();
            assertFatalWithNode_214(
                _node,
                _condition,
                Ptr::<()>::from_string_literal(
                    b"Cannot reset a node which still has children attached",
                ),
            )
        });
        ({
            let _node: Ptr<facebook_yoga_Node> = (*self).clone();
            let _condition: () = ((*self).with(|__s| __s.owner_.clone())).is_null();
            assertFatalWithNode_214(
                _node,
                _condition,
                Ptr::<()>::from_string_literal(b"Cannot reset a node still attached to a owner"),
            )
        });
        (*self).write(facebook_yoga_Node::new_3({
            ({ facebook_yoga_NodeImpl::getConfig(self) })
        }));
    }
}
pub trait facebook_yoga_SmallValueBuffer_4_Impl {
    fn push_4(&self, value: ()) -> ();
    fn push_5(&self, mut value: ()) -> () {
        unimplemented!()
    }
    fn replace_6(&self, index: (), value: ()) -> ();
    fn replace_7(&self, mut index: (), mut value: ()) -> () {
        unimplemented!()
    }
    fn get32(&self, index: ()) -> ();
    fn get64(&self, mut index: ()) -> () {
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
    fn push_4(&self, mut value: ()) -> () {
        let mut index: () = field!((*self), count_).with_mut(|__v| __v.postfix_inc());
        unimplemented!();
        if (index < unimplemented!()) {
            unimplemented!().write(value);
            return index;
        }
        if unimplemented!() {
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        return index;
    }
    fn replace_6(&self, mut index: (), mut value: ()) -> () {
        if (index < unimplemented!()) {
            unimplemented!().write(value);
        } else {
            unimplemented!().write(value);
        }
        return index;
    }
    fn get32(&self, mut index: ()) -> () {
        if (index < unimplemented!()) {
            return (unimplemented!().read());
        } else {
            return (unimplemented!().read());
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
        unimplemented!();
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
        unimplemented!();
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
        unimplemented!();
        unimplemented!();
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
    fn gridTemplateColumns(&self) -> Ptr<()>;
    fn setGridTemplateColumns(&self, value: ());
    fn resizeGridTemplateColumns(&self, count: ());
    fn setGridTemplateColumnAt(&self, index: (), value: facebook_yoga_GridTrackSize);
    fn gridTemplateRows(&self) -> Ptr<()>;
    fn setGridTemplateRows(&self, value: ());
    fn resizeGridTemplateRows(&self, count: ());
    fn setGridTemplateRowAt(&self, index: (), value: facebook_yoga_GridTrackSize);
    fn gridAutoColumns(&self) -> Ptr<()>;
    fn setGridAutoColumns(&self, value: ());
    fn resizeGridAutoColumns(&self, count: ());
    fn setGridAutoColumnAt(&self, index: (), value: facebook_yoga_GridTrackSize);
    fn gridAutoRows(&self) -> Ptr<()>;
    fn setGridAutoRows(&self, value: ());
    fn resizeGridAutoRows(&self, count: ());
    fn setGridAutoRowAt(&self, index: (), value: facebook_yoga_GridTrackSize);
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
        referenceLength: (),
        ownerWidth: (),
    ) -> facebook_yoga_FloatOptional;
    fn maxDimension(&self, axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength;
    fn setMaxDimension(&self, axis: facebook_yoga_Dimension, value: facebook_yoga_StyleSizeLength);
    fn resolvedMaxDimension(
        &self,
        direction: facebook_yoga_Direction,
        axis: facebook_yoga_Dimension,
        referenceLength: (),
        ownerWidth: (),
    ) -> facebook_yoga_FloatOptional;
    fn resolvesAgainstOwnerWidth(&self) -> ();
    fn resolvesAgainstOwnerHeight(&self) -> ();
    fn aspectRatio(&self) -> facebook_yoga_FloatOptional;
    fn setAspectRatio(&self, value: facebook_yoga_FloatOptional);
    fn boxSizing(&self) -> facebook_yoga_BoxSizing;
    fn setBoxSizing(&self, value: facebook_yoga_BoxSizing);
    fn horizontalInsetsDefined(&self) -> ();
    fn verticalInsetsDefined(&self) -> ();
    fn isFlexStartPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isFlexStartPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isInlineStartPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isInlineStartPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isFlexEndPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isFlexEndPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isInlineEndPositionDefined(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn isInlineEndPositionAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn computeFlexStartPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: (),
    ) -> ();
    fn computeInlineStartPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: (),
    ) -> ();
    fn computeFlexEndPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: (),
    ) -> ();
    fn computeInlineEndPosition(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        axisSize: (),
    ) -> ();
    fn computeFlexStartMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeInlineStartMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeFlexEndMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeInlineEndMargin(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeFlexStartBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn computeInlineStartBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn computeFlexEndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn computeInlineEndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn computeFlexStartPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeInlineStartPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeFlexEndPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeInlineEndPadding(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeInlineStartPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeFlexStartPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeInlineEndPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computeFlexEndPaddingAndBorder(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
        widthSize: (),
    ) -> ();
    fn computePaddingAndBorderForDimension(
        &self,
        direction: facebook_yoga_Direction,
        dimension: facebook_yoga_Dimension,
        widthSize: (),
    ) -> ();
    fn computeBorderForAxis(&self, axis: facebook_yoga_FlexDirection) -> ();
    fn computeMarginForAxis(&self, axis: facebook_yoga_FlexDirection, widthSize: ()) -> ();
    fn computeGapForAxis(&self, axis: facebook_yoga_FlexDirection, ownerSize: ()) -> ();
    fn computeGapForDimension(&self, dimension: facebook_yoga_Dimension, ownerSize: ()) -> ();
    fn flexStartMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn flexEndMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn inlineStartMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn inlineEndMarginIsAuto(
        &self,
        axis: facebook_yoga_FlexDirection,
        direction: facebook_yoga_Direction,
    ) -> ();
    fn operator_eq(&self, other: Ptr<facebook_yoga_Style>) -> ();
    fn computeColumnGap(&self) -> facebook_yoga_StyleValueHandle;
    fn computeRowGap(&self) -> facebook_yoga_StyleValueHandle;
    fn computeLeftEdge(
        &self,
        edges: Ptr<()>,
        layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeTopEdge(&self, edges: Ptr<()>) -> facebook_yoga_StyleValueHandle;
    fn computeRightEdge(
        &self,
        edges: Ptr<()>,
        layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle;
    fn computeBottomEdge(&self, edges: Ptr<()>) -> facebook_yoga_StyleValueHandle;
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
        referenceLength: (),
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
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setMargin(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn position(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setPosition(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn padding(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setPadding(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn border(&self, mut edge: facebook_yoga_Edge) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setBorder(&self, mut edge: facebook_yoga_Edge, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn gap(&self, mut gutter: facebook_yoga_Gutter) -> facebook_yoga_StyleLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getLength(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setGap(&self, mut gutter: facebook_yoga_Gutter, mut value: facebook_yoga_StyleLength) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_1(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn dimension(&self, mut axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_2(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn minDimension(&self, mut axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setMinDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
            facebook_yoga_StyleValuePoolImpl::store_2(
                &field_ptr!((*self), pool_),
                _handle,
                (value).clone(),
            )
        });
    }
    fn gridTemplateColumns(&self) -> Ptr<()> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            templateColumns
        );
    }
    fn setGridTemplateColumns(&self, mut value: ()) {
        unimplemented!();
    }
    fn resizeGridTemplateColumns(&self, mut count: ()) {
        unimplemented!();
    }
    fn setGridTemplateColumnAt(&self, mut index: (), mut value: facebook_yoga_GridTrackSize) {
        unimplemented!().write((value).clone());
    }
    fn gridTemplateRows(&self) -> Ptr<()> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            templateRows
        );
    }
    fn setGridTemplateRows(&self, mut value: ()) {
        unimplemented!();
    }
    fn resizeGridTemplateRows(&self, mut count: ()) {
        unimplemented!();
    }
    fn setGridTemplateRowAt(&self, mut index: (), mut value: facebook_yoga_GridTrackSize) {
        unimplemented!().write((value).clone());
    }
    fn gridAutoColumns(&self) -> Ptr<()> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            autoColumns
        );
    }
    fn setGridAutoColumns(&self, mut value: ()) {
        unimplemented!();
    }
    fn resizeGridAutoColumns(&self, mut count: ()) {
        unimplemented!();
    }
    fn setGridAutoColumnAt(&self, mut index: (), mut value: facebook_yoga_GridTrackSize) {
        unimplemented!().write((value).clone());
    }
    fn gridAutoRows(&self) -> Ptr<()> {
        return field_ptr!(
            ({ facebook_yoga_GridStyleStorageImpl::get(&field_ptr!((*self), grid_),) }),
            autoRows
        );
    }
    fn setGridAutoRows(&self, mut value: ()) {
        unimplemented!();
    }
    fn resizeGridAutoRows(&self, mut count: ()) {
        unimplemented!();
    }
    fn setGridAutoRowAt(&self, mut index: (), mut value: facebook_yoga_GridTrackSize) {
        unimplemented!().write((value).clone());
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
        mut referenceLength: (),
        mut ownerWidth: (),
    ) -> facebook_yoga_FloatOptional {
        let handle: Value<facebook_yoga_StyleValueHandle> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
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
            operator_add_111(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    fn maxDimension(&self, mut axis: facebook_yoga_Dimension) -> facebook_yoga_StyleSizeLength {
        return ({
            let _handle: facebook_yoga_StyleValueHandle =
                (*unimplemented!().upgrade().deref()).clone();
            facebook_yoga_StyleValuePoolImpl::getSize(&field_ptr!((*self), pool_), _handle)
        });
    }
    fn setMaxDimension(
        &self,
        mut axis: facebook_yoga_Dimension,
        mut value: facebook_yoga_StyleSizeLength,
    ) {
        ({
            let _handle: Ptr<facebook_yoga_StyleValueHandle> = unimplemented!();
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
        mut referenceLength: (),
        mut ownerWidth: (),
    ) -> facebook_yoga_FloatOptional {
        let handle: Value<facebook_yoga_StyleValueHandle> =
            Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
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
            operator_add_111(
                _lhs,
                (if ({
                    facebook_yoga_FloatOptionalImpl::isDefined(
                        &dimensionPaddingAndBorder.as_pointer(),
                    )
                }) {
                    (*dimensionPaddingAndBorder.borrow()).clone()
                } else {
                    facebook_yoga_FloatOptional::new_1({ 0_f64 })
                }),
            )
        });
    }
    fn resolvesAgainstOwnerWidth(&self) -> () {
        return (((({ facebook_yoga_Style::anyIsPercent_121_9(field_ptr!((*self), margin_)) })
            || ({ facebook_yoga_Style::anyIsPercent_121_9(field_ptr!((*self), padding_)) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isPercent(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isPercent(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isPercent(&unimplemented!()) });
    }
    fn resolvesAgainstOwnerHeight(&self) -> () {
        return (({ facebook_yoga_StyleValueHandleImpl::isPercent(&unimplemented!()) })
            || ({ facebook_yoga_StyleValueHandleImpl::isPercent(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isPercent(&unimplemented!()) });
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
                    operator_eq_109(_lhs, 0_f32)
                }) || (unimplemented!())
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
    fn horizontalInsetsDefined(&self) -> () {
        return ((((({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) })
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) });
    }
    fn verticalInsetsDefined(&self) -> () {
        return ((({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) })
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }))
            || ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) });
    }
    fn isFlexStartPositionDefined(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexStartEdge_127(axis) }),
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexStartEdge_127(axis) }),
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineStartEdge_129(axis, direction) });
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineStartEdge_129(axis, direction) });
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexEndEdge_128(axis) }),
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computePosition(
                            self,
                            ({ flexEndEdge_128(axis) }),
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isDefined(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineEndEdge_130(axis, direction) });
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineEndEdge_130(axis, direction) });
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
        mut axisSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computePosition(
                                self,
                                ({ flexStartEdge_127(axis) }),
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
        mut axisSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineStartEdge_129(axis, direction) });
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
        mut axisSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computePosition(
                                self,
                                ({ flexEndEdge_128(axis) }),
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
        mut axisSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineEndEdge_130(axis, direction) });
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
        mut widthSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computeMargin(
                                self,
                                ({ flexStartEdge_127(axis) }),
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
        mut widthSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineStartEdge_129(axis, direction) });
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
        mut widthSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            facebook_yoga_StyleImpl::computeMargin(
                                self,
                                ({ flexEndEdge_128(axis) }),
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
        mut widthSize: (),
    ) -> () {
        return ({
            facebook_yoga_FloatOptionalImpl::unwrapOrDefault(
                &Rc::new(RefCell::new(
                    ({
                        let _handle: facebook_yoga_StyleValueHandle = ({
                            let _edge: facebook_yoga_PhysicalEdge =
                                ({ inlineEndEdge_130(axis, direction) });
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
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computeBorder(
                                        self,
                                        ({ flexStartEdge_127(axis) }),
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
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineStartEdge_129(axis, direction) });
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
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computeBorder(
                                        self,
                                        ({ flexEndEdge_128(axis) }),
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
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineEndEdge_130(axis, direction) });
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
        mut widthSize: (),
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computePadding(
                                        self,
                                        ({ flexStartEdge_127(axis) }),
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
        mut widthSize: (),
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineStartEdge_129(axis, direction) });
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
        mut widthSize: (),
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    facebook_yoga_StyleImpl::computePadding(
                                        self,
                                        ({ flexEndEdge_128(axis) }),
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
        mut widthSize: (),
    ) -> () {
        return ({
            maxOrDefined_104(
                ({
                    facebook_yoga_FloatOptionalImpl::unwrap(
                        &Rc::new(RefCell::new(
                            ({
                                let _handle: facebook_yoga_StyleValueHandle = ({
                                    let _edge: facebook_yoga_PhysicalEdge =
                                        ({ inlineEndEdge_130(axis, direction) });
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
        mut widthSize: (),
    ) -> () {
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
        mut widthSize: (),
    ) -> () {
        return (({
            facebook_yoga_StyleImpl::computeFlexStartPadding(self, axis, direction, widthSize)
        }) + ({ facebook_yoga_StyleImpl::computeFlexStartBorder(self, axis, direction) }));
    }
    fn computeInlineEndPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (({
            facebook_yoga_StyleImpl::computeInlineEndPadding(self, axis, direction, widthSize)
        }) + ({ facebook_yoga_StyleImpl::computeInlineEndBorder(self, axis, direction) }));
    }
    fn computeFlexEndPaddingAndBorder(
        &self,
        mut axis: facebook_yoga_FlexDirection,
        mut direction: facebook_yoga_Direction,
        mut widthSize: (),
    ) -> () {
        return (({
            facebook_yoga_StyleImpl::computeFlexEndPadding(self, axis, direction, widthSize)
        }) + ({ facebook_yoga_StyleImpl::computeFlexEndBorder(self, axis, direction) }));
    }
    fn computePaddingAndBorderForDimension(
        &self,
        mut direction: facebook_yoga_Direction,
        mut dimension: facebook_yoga_Dimension,
        mut widthSize: (),
    ) -> () {
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
    fn computeBorderForAxis(&self, mut axis: facebook_yoga_FlexDirection) -> () {
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
    fn computeMarginForAxis(&self, mut axis: facebook_yoga_FlexDirection, mut widthSize: ()) -> () {
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
    fn computeGapForAxis(&self, mut axis: facebook_yoga_FlexDirection, mut ownerSize: ()) -> () {
        let mut gap: facebook_yoga_StyleValueHandle = if ({ isRow_123(axis) }) {
            ({ facebook_yoga_StyleImpl::computeColumnGap(self) })
        } else {
            ({ facebook_yoga_StyleImpl::computeRowGap(self) })
        };
        return ({
            maxOrDefined_104(
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
        mut ownerSize: (),
    ) -> () {
        let mut gap: facebook_yoga_StyleValueHandle =
            if (dimension == facebook_yoga_Dimension_Width) {
                ({ facebook_yoga_StyleImpl::computeColumnGap(self) })
            } else {
                ({ facebook_yoga_StyleImpl::computeRowGap(self) })
            };
        return ({
            maxOrDefined_104(
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computeMargin(
                            self,
                            ({ flexStartEdge_127(axis) }),
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        facebook_yoga_StyleImpl::computeMargin(
                            self,
                            ({ flexEndEdge_128(axis) }),
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineStartEdge_129(axis, direction) });
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
    ) -> () {
        return ({
            facebook_yoga_StyleValueHandleImpl::isAuto(
                &Rc::new(RefCell::new(
                    ({
                        let _edge: facebook_yoga_PhysicalEdge =
                            ({ inlineEndEdge_130(axis, direction) });
                        let _direction: facebook_yoga_Direction = direction;
                        facebook_yoga_StyleImpl::computeMargin(self, _edge, _direction)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn operator_eq(&self, other: Ptr<facebook_yoga_Style>) -> () {
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
                let _lhs: Ptr<()> = field_ptr!((*self), margin_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, margin_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), position_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, position_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), padding_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, padding_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), border_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, border_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_9(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), gap_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, gap_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::lengthsEqual_124_3(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), dimensions_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, dimensions_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), minDimensions_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, minDimensions_);
                let _rhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!(other, pool_);
                facebook_yoga_Style::sizeLengthsEqual_126_2(_lhs, _lhsPool, _rhs, _rhsPool)
            }))
            && ({
                let _lhs: Ptr<()> = field_ptr!((*self), maxDimensions_);
                let _lhsPool: Ptr<facebook_yoga_StyleValuePool> = field_ptr!((*self), pool_);
                let _rhs: Ptr<()> = field_ptr!(other, maxDimensions_);
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
        if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else {
            return (*unimplemented!().upgrade().deref()).clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeRowGap(&self) -> facebook_yoga_StyleValueHandle {
        if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else {
            return (*unimplemented!().upgrade().deref()).clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeLeftEdge(
        &self,
        edges: Ptr<()>,
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if (layoutDirection == facebook_yoga_Direction_LTR)
            && ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) })
        {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if (layoutDirection == facebook_yoga_Direction_RTL)
            && ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) })
        {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else {
            return (*unimplemented!().upgrade().deref()).clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeTopEdge(&self, edges: Ptr<()>) -> facebook_yoga_StyleValueHandle {
        if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else {
            return (*unimplemented!().upgrade().deref()).clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeRightEdge(
        &self,
        edges: Ptr<()>,
        mut layoutDirection: facebook_yoga_Direction,
    ) -> facebook_yoga_StyleValueHandle {
        if (layoutDirection == facebook_yoga_Direction_LTR)
            && ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) })
        {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if (layoutDirection == facebook_yoga_Direction_RTL)
            && ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) })
        {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else {
            return (*unimplemented!().upgrade().deref()).clone();
        }
        panic!("ub: non-void function does not return a value")
    }
    fn computeBottomEdge(&self, edges: Ptr<()>) -> facebook_yoga_StyleValueHandle {
        if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else if ({ facebook_yoga_StyleValueHandleImpl::isDefined(&unimplemented!()) }) {
            return (*unimplemented!().upgrade().deref()).clone();
        } else {
            return (*unimplemented!().upgrade().deref()).clone();
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
                    let _edges: Ptr<()> = field_ptr!((*self), position_);
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), position_);
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), position_);
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), position_);
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid physical edge")) });
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
                    let _edges: Ptr<()> = field_ptr!((*self), margin_);
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), margin_);
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), margin_);
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), margin_);
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid physical edge")) });
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
                    let _edges: Ptr<()> = field_ptr!((*self), padding_);
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), padding_);
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), padding_);
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), padding_);
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid physical edge")) });
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
                    let _edges: Ptr<()> = field_ptr!((*self), border_);
                    facebook_yoga_StyleImpl::computeLeftEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Top => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), border_);
                    facebook_yoga_StyleImpl::computeTopEdge(self, _edges)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Right => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), border_);
                    facebook_yoga_StyleImpl::computeRightEdge(self, _edges, direction)
                });
            }
            __v if __v == facebook_yoga_PhysicalEdge_Bottom => {
                return ({
                    let _edges: Ptr<()> = field_ptr!((*self), border_);
                    facebook_yoga_StyleImpl::computeBottomEdge(self, _edges)
                });
            }
            _ => {
                ({ fatalWithMessage_97(Ptr::<()>::from_string_literal(b"Invalid physical edge")) });
            }
        });
        panic!("ub: non-void function does not return a value")
    }
    fn resolve(
        &self,
        handle: facebook_yoga_StyleValueHandle,
        mut referenceLength: (),
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
    fn isAuto(&self) -> ();
    fn isUndefined(&self) -> ();
    fn isPoints(&self) -> ();
    fn isPercent(&self) -> ();
    fn isDefined(&self) -> ();
    fn value(&self) -> facebook_yoga_FloatOptional;
    fn resolve(&self, referenceLength: ()) -> facebook_yoga_FloatOptional;
    fn to_YGValue(&self) -> YGValue;
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleLength>) -> ();
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleLength>) -> ();
}
impl facebook_yoga_StyleLengthImpl for Ptr<facebook_yoga_StyleLength> {
    fn isAuto(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Auto);
    }
    fn isUndefined(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Undefined);
    }
    fn isPoints(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Point);
    }
    fn isPercent(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Percent);
    }
    fn isDefined(&self) -> () {
        return !({ facebook_yoga_StyleLengthImpl::isUndefined(self) });
    }
    fn value(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.value_.clone());
    }
    fn resolve(&self, mut referenceLength: ()) -> facebook_yoga_FloatOptional {
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
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleLength>) -> () {
        return ({
            let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
            let _rhs: facebook_yoga_FloatOptional = rhs.with(|__s| __s.value_.clone());
            operator_eq_108(_lhs, _rhs)
        }) && ({ (*self).with(|__s| __s.unit_) } == { rhs.with(|__s| __s.unit_) });
    }
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleLength>) -> () {
        return ({ (*self).with(|__s| __s.unit_) } == { other.with(|__s| __s.unit_) })
            && ({
                let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
                let _rhs: facebook_yoga_FloatOptional = other.with(|__s| __s.value_.clone());
                inexactEquals_116(_lhs, _rhs)
            });
    }
}
pub trait facebook_yoga_StyleSizeLengthImpl {
    fn isAuto(&self) -> ();
    fn isMaxContent(&self) -> ();
    fn isFitContent(&self) -> ();
    fn isStretch(&self) -> ();
    fn isUndefined(&self) -> ();
    fn isDefined(&self) -> ();
    fn isPoints(&self) -> ();
    fn isPercent(&self) -> ();
    fn value(&self) -> facebook_yoga_FloatOptional;
    fn resolve(&self, referenceLength: ()) -> facebook_yoga_FloatOptional;
    fn to_YGValue(&self) -> YGValue;
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleSizeLength>) -> ();
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleSizeLength>) -> ();
}
impl facebook_yoga_StyleSizeLengthImpl for Ptr<facebook_yoga_StyleSizeLength> {
    fn isAuto(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Auto);
    }
    fn isMaxContent(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_MaxContent);
    }
    fn isFitContent(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_FitContent);
    }
    fn isStretch(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Stretch);
    }
    fn isUndefined(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Undefined);
    }
    fn isDefined(&self) -> () {
        return !({ facebook_yoga_StyleSizeLengthImpl::isUndefined(self) });
    }
    fn isPoints(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Point);
    }
    fn isPercent(&self) -> () {
        return ((*self).with(|__s| __s.unit_) == facebook_yoga_Unit_Percent);
    }
    fn value(&self) -> facebook_yoga_FloatOptional {
        return (*self).with(|__s| __s.value_.clone());
    }
    fn resolve(&self, mut referenceLength: ()) -> facebook_yoga_FloatOptional {
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
    fn operator_eq(&self, rhs: Ptr<facebook_yoga_StyleSizeLength>) -> () {
        return ({
            let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
            let _rhs: facebook_yoga_FloatOptional = rhs.with(|__s| __s.value_.clone());
            operator_eq_108(_lhs, _rhs)
        }) && ({ (*self).with(|__s| __s.unit_) } == { rhs.with(|__s| __s.unit_) });
    }
    fn inexactEquals(&self, other: Ptr<facebook_yoga_StyleSizeLength>) -> () {
        return ({ (*self).with(|__s| __s.unit_) } == { other.with(|__s| __s.unit_) })
            && ({
                let _lhs: facebook_yoga_FloatOptional = (*self).with(|__s| __s.value_.clone());
                let _rhs: facebook_yoga_FloatOptional = other.with(|__s| __s.value_.clone());
                inexactEquals_116(_lhs, _rhs)
            });
    }
}
pub trait facebook_yoga_StyleValueHandleImpl {
    fn isUndefined(&self) -> ();
    fn isDefined(&self) -> ();
    fn isAuto(&self) -> ();
    fn isPercent(&self) -> ();
    fn isPoint(&self) -> ();
    fn isKeyword(&self, keyword: facebook_yoga_StyleValueHandle_Keyword) -> ();
    fn type_(&self) -> facebook_yoga_StyleValueHandle_Type;
    fn setType(&self, handleType: facebook_yoga_StyleValueHandle_Type);
    fn value(&self) -> ();
    fn setValue(&self, value: ());
    fn isValueIndexed(&self) -> ();
    fn setValueIsIndexed(&self);
}
impl facebook_yoga_StyleValueHandleImpl for Ptr<facebook_yoga_StyleValueHandle> {
    fn isUndefined(&self) -> () {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Undefined);
    }
    fn isDefined(&self) -> () {
        return !({ facebook_yoga_StyleValueHandleImpl::isUndefined(self) });
    }
    fn isAuto(&self) -> () {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Auto);
    }
    fn isPercent(&self) -> () {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Percent);
    }
    fn isPoint(&self) -> () {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Point);
    }
    fn isKeyword(&self, mut keyword: facebook_yoga_StyleValueHandle_Keyword) -> () {
        return (({ facebook_yoga_StyleValueHandleImpl::type_(self) })
            == facebook_yoga_StyleValueHandle_Type_Keyword)
            && (({ facebook_yoga_StyleValueHandleImpl::value(self) }) == (keyword as ()));
    }
    fn type_(&self) -> facebook_yoga_StyleValueHandle_Type {
        return (((*self).with(|__s| __s.repr_) & kHandleTypeMask_175.with(|rc| *rc.borrow()))
            as facebook_yoga_StyleValueHandle_Type);
    }
    fn setType(&self, mut handleType: facebook_yoga_StyleValueHandle_Type) {
        {
            field!((*self), repr_)
                .with_mut(|__v| *__v = *__v & (!kHandleTypeMask_175.with(|rc| *rc.borrow())))
        };
        {
            let __rhs = (handleType as ());
            field!((*self), repr_).with_mut(|__v| *__v = *__v | __rhs)
        };
    }
    fn value(&self) -> () {
        return ((*self).with(|__s| __s.repr_) >> 4_());
    }
    fn setValue(&self, mut value: ()) {
        {
            field!((*self), repr_)
                .with_mut(|__v| *__v = *__v & (!kHandleValueMask_177.with(|rc| *rc.borrow())))
        };
        {
            let __rhs = (value << 4_());
            field!((*self), repr_).with_mut(|__v| *__v = *__v | __rhs)
        };
    }
    fn isValueIndexed(&self) -> () {
        return (((*self).with(|__s| __s.repr_) & kHandleIndexedMask_176.with(|rc| *rc.borrow()))
            != 0_());
    }
    fn setValueIsIndexed(&self) {
        {
            field!((*self), repr_)
                .with_mut(|__v| *__v = *__v | kHandleIndexedMask_176.with(|rc| *rc.borrow()))
        };
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
    fn getStoredValue(&self, handle: facebook_yoga_StyleValueHandle) -> ();
    fn storeValue(
        &self,
        handle: Ptr<facebook_yoga_StyleValueHandle>,
        value: (),
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
                let _value: () = ({
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
                let _value: () = ({
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
                let _value: () =
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
            unimplemented!();
            let mut value: () =
                if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer()) }) {
                    unimplemented!()
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
            unimplemented!();
            let mut value: () =
                if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer()) }) {
                    unimplemented!()
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
            unimplemented!();
            let mut value: () =
                if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer()) }) {
                    unimplemented!()
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
    fn getStoredValue(&self, handle: facebook_yoga_StyleValueHandle) -> () {
        let handle: Value<facebook_yoga_StyleValueHandle> = Rc::new(RefCell::new(handle));
        unimplemented!();
        return if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle.as_pointer()) }) {
            unimplemented!()
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
        mut value: (),
        mut type_: facebook_yoga_StyleValueHandle_Type,
    ) {
        ({
            let _handleType: facebook_yoga_StyleValueHandle_Type = type_;
            facebook_yoga_StyleValueHandleImpl::setType(&handle, _handleType)
        });
        if ({ facebook_yoga_StyleValueHandleImpl::isValueIndexed(&handle) }) {
            let mut newIndex: () = ({
                facebook_yoga_SmallValueBuffer_4_Impl::replace_6(
                    &field_ptr!((*self), buffer_),
                    ({ facebook_yoga_StyleValueHandleImpl::value(&handle) }),
                    unimplemented!(),
                )
            });
            ({
                let _value: () = newIndex;
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        } else if ({ facebook_yoga_StyleValuePool::isIntegerPackable(value) }) {
            ({
                let _value: () = ({ facebook_yoga_StyleValuePool::packInlineInteger(value) });
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        } else {
            let mut newIndex: () = ({
                facebook_yoga_SmallValueBuffer_4_Impl::push_4(
                    &field_ptr!((*self), buffer_),
                    unimplemented!(),
                )
            });
            ({
                let _value: () = newIndex;
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
            let mut newIndex: () = ({
                facebook_yoga_SmallValueBuffer_4_Impl::replace_6(
                    &field_ptr!((*self), buffer_),
                    ({ facebook_yoga_StyleValueHandleImpl::value(&handle) }),
                    (keyword as ()),
                )
            });
            ({
                let _value: () = newIndex;
                facebook_yoga_StyleValueHandleImpl::setValue(&handle, _value)
            });
        } else {
            ({
                let _value: () = (keyword as ());
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
pub fn __cpp2rust_init_globals() {
    let _ = YGUndefined_24.with(|_| ());
    let _ = defaultConfig__62.with(|_| ());
    let _ = MaxCachedMeasurements_117.with(|_| ());
    let _ = kHandleTypeMask_175.with(|_| ());
    let _ = kHandleIndexedMask_176.with(|_| ());
    let _ = kHandleValueMask_177.with(|_| ());
    let _ = DefaultFlexGrow_178.with(|_| ());
    let _ = DefaultFlexShrink_179.with(|_| ());
    let _ = WebDefaultFlexShrink_180.with(|_| ());
    let _ = YGValueZero_2.with(|_| ());
    let _ = YGValueUndefined_1.with(|_| ());
    let _ = YGValueAuto_0.with(|_| ());
    let _ = gCurrentGenerationCount_440.with(|_| ());
    let _ = subscribers_479.with(|_| ());
}
