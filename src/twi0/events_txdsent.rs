#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_TXDSENT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `EVENTS_TXDSENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_TXDSENTR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_TXDSENTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EVENTS_TXDSENTR::NOTGENERATED => false,
            EVENTS_TXDSENTR::GENERATED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVENTS_TXDSENTR {
        match value {
            false => EVENTS_TXDSENTR::NOTGENERATED,
            true => EVENTS_TXDSENTR::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXDSENTR::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXDSENTR::GENERATED
    }
}
#[doc = "Values that can be written to the field `EVENTS_TXDSENT`"]
pub enum EVENTS_TXDSENTW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_TXDSENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTS_TXDSENTW::NOTGENERATED => false,
            EVENTS_TXDSENTW::GENERATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVENTS_TXDSENTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_TXDSENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVENTS_TXDSENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Event not generated"]
    #[inline]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXDSENTW::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXDSENTW::GENERATED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TWI TXD byte sent"]
    #[inline]
    pub fn events_txdsent(&self) -> EVENTS_TXDSENTR {
        EVENTS_TXDSENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TWI TXD byte sent"]
    #[inline]
    pub fn events_txdsent(&mut self) -> _EVENTS_TXDSENTW {
        _EVENTS_TXDSENTW { w: self }
    }
}