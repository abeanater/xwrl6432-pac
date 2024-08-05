#[doc = "Register `BLOCKRESET2` reader"]
pub type R = crate::R<Blockreset2Spec>;
#[doc = "Register `BLOCKRESET2` writer"]
pub type W = crate::W<Blockreset2Spec>;
#[doc = "Field `rs232` reader - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type Rs232R = crate::FieldReader;
#[doc = "Field `rs232` writer - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type Rs232W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `frc` reader - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type FrcR = crate::FieldReader;
#[doc = "Field `frc` writer - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
pub type FrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn rs232(&self) -> Rs232R {
        Rs232R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    pub fn frc(&self) -> FrcR {
        FrcR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn rs232(&mut self) -> Rs232W<Blockreset2Spec> {
        Rs232W::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Release the reset 0x7 : Assert the reset"]
    #[inline(always)]
    #[must_use]
    pub fn frc(&mut self) -> FrcW<Blockreset2Spec> {
        FrcW::new(self, 6)
    }
}
#[doc = "BLOCKRESET2\n\nYou can [`read`](crate::Reg::read) this register and get [`blockreset2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockreset2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Blockreset2Spec;
impl crate::RegisterSpec for Blockreset2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blockreset2::R`](R) reader structure"]
impl crate::Readable for Blockreset2Spec {}
#[doc = "`write(|w| ..)` method takes [`blockreset2::W`](W) writer structure"]
impl crate::Writable for Blockreset2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLOCKRESET2 to value 0"]
impl crate::Resettable for Blockreset2Spec {
    const RESET_VALUE: u32 = 0;
}
