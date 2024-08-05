#[doc = "Register `RS232_BITINTERVAL_2_3` reader"]
pub type R = crate::R<Rs232Bitinterval2_3Spec>;
#[doc = "Register `RS232_BITINTERVAL_2_3` writer"]
pub type W = crate::W<Rs232Bitinterval2_3Spec>;
#[doc = "Field `rs232_bitinterval_2` reader - 11:0\\]
BIT Interval value for 26MHz XTAL"]
pub type Rs232Bitinterval2R = crate::FieldReader<u16>;
#[doc = "Field `rs232_bitinterval_2` writer - 11:0\\]
BIT Interval value for 26MHz XTAL"]
pub type Rs232Bitinterval2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `rs232_bitinterval_3` reader - 27:16\\]
BIT Interval value for 38.4MHz XTAL"]
pub type Rs232Bitinterval3R = crate::FieldReader<u16>;
#[doc = "Field `rs232_bitinterval_3` writer - 27:16\\]
BIT Interval value for 38.4MHz XTAL"]
pub type Rs232Bitinterval3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
BIT Interval value for 26MHz XTAL"]
    #[inline(always)]
    pub fn rs232_bitinterval_2(&self) -> Rs232Bitinterval2R {
        Rs232Bitinterval2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
BIT Interval value for 38.4MHz XTAL"]
    #[inline(always)]
    pub fn rs232_bitinterval_3(&self) -> Rs232Bitinterval3R {
        Rs232Bitinterval3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
BIT Interval value for 26MHz XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn rs232_bitinterval_2(&mut self) -> Rs232Bitinterval2W<Rs232Bitinterval2_3Spec> {
        Rs232Bitinterval2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
BIT Interval value for 38.4MHz XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn rs232_bitinterval_3(&mut self) -> Rs232Bitinterval3W<Rs232Bitinterval2_3Spec> {
        Rs232Bitinterval3W::new(self, 16)
    }
}
#[doc = "RS232_BITINTERVAL_2_3\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_bitinterval_2_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_bitinterval_2_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs232Bitinterval2_3Spec;
impl crate::RegisterSpec for Rs232Bitinterval2_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs232_bitinterval_2_3::R`](R) reader structure"]
impl crate::Readable for Rs232Bitinterval2_3Spec {}
#[doc = "`write(|w| ..)` method takes [`rs232_bitinterval_2_3::W`](W) writer structure"]
impl crate::Writable for Rs232Bitinterval2_3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS232_BITINTERVAL_2_3 to value 0"]
impl crate::Resettable for Rs232Bitinterval2_3Spec {
    const RESET_VALUE: u32 = 0;
}
