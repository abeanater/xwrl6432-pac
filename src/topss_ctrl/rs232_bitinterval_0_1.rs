#[doc = "Register `RS232_BITINTERVAL_0_1` reader"]
pub type R = crate::R<Rs232Bitinterval0_1Spec>;
#[doc = "Register `RS232_BITINTERVAL_0_1` writer"]
pub type W = crate::W<Rs232Bitinterval0_1Spec>;
#[doc = "Field `rs232_bitinterval_0` reader - 11:0\\]
BIT Interval value for 25MHz XTAL"]
pub type Rs232Bitinterval0R = crate::FieldReader<u16>;
#[doc = "Field `rs232_bitinterval_0` writer - 11:0\\]
BIT Interval value for 25MHz XTAL"]
pub type Rs232Bitinterval0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `rs232_bitinterval_1` reader - 27:16\\]
BIT Interval value for 40MHz XTAL"]
pub type Rs232Bitinterval1R = crate::FieldReader<u16>;
#[doc = "Field `rs232_bitinterval_1` writer - 27:16\\]
BIT Interval value for 40MHz XTAL"]
pub type Rs232Bitinterval1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
BIT Interval value for 25MHz XTAL"]
    #[inline(always)]
    pub fn rs232_bitinterval_0(&self) -> Rs232Bitinterval0R {
        Rs232Bitinterval0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
BIT Interval value for 40MHz XTAL"]
    #[inline(always)]
    pub fn rs232_bitinterval_1(&self) -> Rs232Bitinterval1R {
        Rs232Bitinterval1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
BIT Interval value for 25MHz XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn rs232_bitinterval_0(&mut self) -> Rs232Bitinterval0W<Rs232Bitinterval0_1Spec> {
        Rs232Bitinterval0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
BIT Interval value for 40MHz XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn rs232_bitinterval_1(&mut self) -> Rs232Bitinterval1W<Rs232Bitinterval0_1Spec> {
        Rs232Bitinterval1W::new(self, 16)
    }
}
#[doc = "RS232_BITINTERVAL_0_1\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_bitinterval_0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_bitinterval_0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs232Bitinterval0_1Spec;
impl crate::RegisterSpec for Rs232Bitinterval0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs232_bitinterval_0_1::R`](R) reader structure"]
impl crate::Readable for Rs232Bitinterval0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`rs232_bitinterval_0_1::W`](W) writer structure"]
impl crate::Writable for Rs232Bitinterval0_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS232_BITINTERVAL_0_1 to value 0"]
impl crate::Resettable for Rs232Bitinterval0_1Spec {
    const RESET_VALUE: u32 = 0;
}
