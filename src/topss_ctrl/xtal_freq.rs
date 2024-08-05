#[doc = "Register `XTAL_FREQ` reader"]
pub type R = crate::R<XtalFreqSpec>;
#[doc = "Register `XTAL_FREQ` writer"]
pub type W = crate::W<XtalFreqSpec>;
#[doc = "Field `clkm_xtal_freq` reader - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
pub type ClkmXtalFreqR = crate::FieldReader;
#[doc = "Field `clkm_xtal_freq` writer - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
pub type ClkmXtalFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
    #[inline(always)]
    pub fn clkm_xtal_freq(&self) -> ClkmXtalFreqR {
        ClkmXtalFreqR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_freq(&mut self) -> ClkmXtalFreqW<XtalFreqSpec> {
        ClkmXtalFreqW::new(self, 0)
    }
}
#[doc = "XTAL_FREQ\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_freq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_freq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalFreqSpec;
impl crate::RegisterSpec for XtalFreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_freq::R`](R) reader structure"]
impl crate::Readable for XtalFreqSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_freq::W`](W) writer structure"]
impl crate::Writable for XtalFreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTAL_FREQ to value 0"]
impl crate::Resettable for XtalFreqSpec {
    const RESET_VALUE: u32 = 0;
}
