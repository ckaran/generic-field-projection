//! Experimental support for pinnable pointers

use super::*;

impl<'a, F: Field, P> ProjectTo<PinToPin<F>> for Pin<P>
where
    P: PinnablePointer + ProjectTo<F>,
    P::Projection: core::ops::Deref<Target = F::Type>,
{
    type Projection = Pin<P::Projection>;

    fn project_to(self, pin_field: PinToPin<F>) -> Self::Projection {
        unsafe {
            let inner = Pin::into_inner_unchecked(self);

            Pin::new_unchecked(inner.project_to(pin_field.field()))
        }
    }
}

impl<'a, F: Field, P> ProjectTo<PinToPtr<F>> for Pin<P>
where
    // TODO: I don't know if `PinnablePointer` is strictly required here
    P: PinnablePointer + ProjectTo<F>,
{
    type Projection = P::Projection;

    fn project_to(self, pin_field: PinToPtr<F>) -> Self::Projection {
        unsafe {
            let inner = Pin::into_inner_unchecked(self);

            inner.project_to(pin_field.0)
        }
    }
}

pub struct MakePin;
pub struct MakePtr;

pub struct CreateTag;

pub struct BuildOutput;

type_function! {
    for(F: Field) fn(self: CreateTag, _pin_to_pin: PinToPin<F>) -> MakePin {
        MakePin
    }

    for(F: Field) fn(self: CreateTag, _pin_to_ptr: PinToPtr<F>) -> MakePtr {
        MakePtr
    }

    for(T: Deref) fn(self: BuildOutput, MakePin: MakePin, value: T) -> Pin<T> {
        unsafe { Pin::new_unchecked(value) }
    }

    for(T) fn(self: BuildOutput, MakePtr: MakePtr, value: T) -> T {
        value
    }
}

impl<F: Copy + FieldSet, P> ProjectToSet<F> for Pin<P>
where
    P: PinnablePointer + ProjectToSet<F>,
    F: TupleMap<CreateTag>,
    TMap<F, CreateTag>: TupleZip<P::Projection, BuildOutput>,
{
    type Projection = TZip<TMap<F, CreateTag>, P::Projection, BuildOutput>;

    #[inline]
    fn project_set_to(self, field: F) -> Self::Projection {
        unsafe {
            let tags = field.tup_map(CreateTag);

            let raw_output =
                Pin::into_inner_unchecked(self).project_set_to(field);

            tags.tup_zip(raw_output, BuildOutput)
        }
    }
}
