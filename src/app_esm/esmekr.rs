#[doc = "Register `ESMEKR` reader"]
pub type R = crate::R<EsmekrSpec>;
#[doc = "Register `ESMEKR` writer"]
pub type W = crate::W<EsmekrSpec>;
#[doc = "Field `EKEY` reader - 3:0\\]
Error Key. The key to reset the ERROR pin or to force an error on the ERROR pin. User and privileged mode (read): Returns current value of the EKEY. Privileged mode (write): 0 Activates normal mode (recommended default mode). Ah Forces error on ERROR pin. 5h The ERROR pin set to high when the low time counter (LTC) has completed; then the EKEY bit will switch back to normal mode (EKEY = 0000) All other values Activates normal mode."]
pub type EkeyR = crate::FieldReader;
#[doc = "Field `EKEY` writer - 3:0\\]
Error Key. The key to reset the ERROR pin or to force an error on the ERROR pin. User and privileged mode (read): Returns current value of the EKEY. Privileged mode (write): 0 Activates normal mode (recommended default mode). Ah Forces error on ERROR pin. 5h The ERROR pin set to high when the low time counter (LTC) has completed; then the EKEY bit will switch back to normal mode (EKEY = 0000) All other values Activates normal mode."]
pub type EkeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Error Key. The key to reset the ERROR pin or to force an error on the ERROR pin. User and privileged mode (read): Returns current value of the EKEY. Privileged mode (write): 0 Activates normal mode (recommended default mode). Ah Forces error on ERROR pin. 5h The ERROR pin set to high when the low time counter (LTC) has completed; then the EKEY bit will switch back to normal mode (EKEY = 0000) All other values Activates normal mode."]
    #[inline(always)]
    pub fn ekey(&self) -> EkeyR {
        EkeyR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Error Key. The key to reset the ERROR pin or to force an error on the ERROR pin. User and privileged mode (read): Returns current value of the EKEY. Privileged mode (write): 0 Activates normal mode (recommended default mode). Ah Forces error on ERROR pin. 5h The ERROR pin set to high when the low time counter (LTC) has completed; then the EKEY bit will switch back to normal mode (EKEY = 0000) All other values Activates normal mode."]
    #[inline(always)]
    #[must_use]
    pub fn ekey(&mut self) -> EkeyW<EsmekrSpec> {
        EkeyW::new(self, 0)
    }
}
#[doc = "ESM Error Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmekr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmekr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmekrSpec;
impl crate::RegisterSpec for EsmekrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmekr::R`](R) reader structure"]
impl crate::Readable for EsmekrSpec {}
#[doc = "`write(|w| ..)` method takes [`esmekr::W`](W) writer structure"]
impl crate::Writable for EsmekrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMEKR to value 0"]
impl crate::Resettable for EsmekrSpec {
    const RESET_VALUE: u32 = 0;
}
