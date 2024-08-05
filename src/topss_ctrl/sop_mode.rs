#[doc = "Register `SOP_MODE` reader"]
pub type R = crate::R<SopModeSpec>;
#[doc = "Register `SOP_MODE` writer"]
pub type W = crate::W<SopModeSpec>;
#[doc = "Field `sop_mode` reader - 1:0\\]
SOP MODE, 0x0 = Device Management Mode 0x1 = Application Mode 0x2 = Test mode 0x3 = Debug Mode"]
pub type SopModeR = crate::FieldReader;
#[doc = "Field `sop_mode` writer - 1:0\\]
SOP MODE, 0x0 = Device Management Mode 0x1 = Application Mode 0x2 = Test mode 0x3 = Debug Mode"]
pub type SopModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
SOP MODE, 0x0 = Device Management Mode 0x1 = Application Mode 0x2 = Test mode 0x3 = Debug Mode"]
    #[inline(always)]
    pub fn sop_mode(&self) -> SopModeR {
        SopModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
SOP MODE, 0x0 = Device Management Mode 0x1 = Application Mode 0x2 = Test mode 0x3 = Debug Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sop_mode(&mut self) -> SopModeW<SopModeSpec> {
        SopModeW::new(self, 0)
    }
}
#[doc = "SOP_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`sop_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sop_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SopModeSpec;
impl crate::RegisterSpec for SopModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sop_mode::R`](R) reader structure"]
impl crate::Readable for SopModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sop_mode::W`](W) writer structure"]
impl crate::Writable for SopModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOP_MODE to value 0"]
impl crate::Resettable for SopModeSpec {
    const RESET_VALUE: u32 = 0;
}
