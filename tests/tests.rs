use derivative::Derivative;
use vargs::vargs;

#[test]
fn test_vargs() {
    vargs! {
        #[derive(Default)]
        Vargs {
            one: u32,
            two: i32
        }
    }

    fn test<V: Into<Vargs>>(vargs: V) -> (u32, i32) {
        let vargs = vargs.into();
        (vargs.one, vargs.two)
    }

    let (one, two) = test(());
    assert_eq!(0, one);
    assert_eq!(0, two);

    let (one, two) = test(None);
    assert_eq!(0, one);
    assert_eq!(0, two);

    let (one, two) = test(1);
    assert_eq!(1, one);
    assert_eq!(0, two);

    let (one, two) = test((1, 2));
    assert_eq!(1, one);
    assert_eq!(2, two);
}

#[test]
fn test_derivative() {
    vargs! {
        #[derive(Derivative)]
        #[derivative(Default)]
        Vargs {
            #[derivative(Default(value="1"))]
            one: u32,
            #[derivative(Default(value="2"))]
            two: i32
        }
    }

    fn test<V: Into<Vargs>>(vargs: V) -> (u32, i32) {
        let vargs = vargs.into();
        (vargs.one, vargs.two)
    }

    let (one, two) = test(());
    assert_eq!(1, one);
    assert_eq!(2, two);

    let (one, two) = test(None);
    assert_eq!(1, one);
    assert_eq!(2, two);

    let (one, two) = test(11);
    assert_eq!(11, one);
    assert_eq!(2, two);

    let (one, two) = test((11, 22));
    assert_eq!(11, one);
    assert_eq!(22, two);
}

#[test]
fn test_lifetime_annotated_option() {
    vargs! {
        #[derive(Default)]
        Vargs<'a, 'b> {
            @[option]
            #[doc = "attributes under @[option]"]
            one: &'a u32,

            @[option]
            #[doc = "attributes under @[option]"]
            two: &'b i32
        }
    }

    fn test<'a, 'b, V: Into<Vargs<'a, 'b>>>(vargs: V) -> (Option<&'a u32>, Option<&'b i32>) {
        let vargs = vargs.into();
        (vargs.one, vargs.two)
    }

    let (one, two) = test(());
    assert_eq!(None, one);
    assert_eq!(None, two);

    let (one, two) = test(None);
    assert_eq!(None, one);
    assert_eq!(None, two);

    let (one, two) = test(&1u32);
    assert_eq!(Some(&1), one);
    assert_eq!(None, two);

    let (one, two) = test((&1u32, &2i32));
    assert_eq!(Some(&1), one);
    assert_eq!(Some(&2), two);
}

#[test]
fn test_lifetime_explicit_option() {
    vargs! {
        #[derive(Default)]
        Vargs<'a, 'b> {
            one: Option<&'a u32>,
            two: Option<&'b i32>
        }
    }

    fn test<'a, 'b, V: Into<Vargs<'a, 'b>>>(vargs: V) -> (Option<&'a u32>, Option<&'b i32>) {
        let vargs = vargs.into();
        (vargs.one, vargs.two)
    }

    let (one, two) = test(());
    assert_eq!(None, one);
    assert_eq!(None, two);

    let (one, two) = test(Option::<Vargs>::None);
    assert_eq!(None, one);
    assert_eq!(None, two);

    let (one, two) = test(Some(&1u32));
    assert_eq!(Some(&1), one);
    assert_eq!(None, two);

    let (one, two) = test((Some(&1u32), Some(&2i32)));
    assert_eq!(Some(&1), one);
    assert_eq!(Some(&2), two);
}