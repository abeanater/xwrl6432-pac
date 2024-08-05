#[doc = "Register `REG2` reader"]
pub type R = crate::R<Reg2Spec>;
#[doc = "Register `REG2` writer"]
pub type W = crate::W<Reg2Spec>;
#[doc = "Field `CONFIG_VALUE_IFM` reader - 31:0\\]
Configuration value to be passed to analog in IFM mode"]
pub type ConfigValueIfmR = crate::FieldReader<u32>;
#[doc = "Field `CONFIG_VALUE_IFM` writer - 31:0\\]
Configuration value to be passed to analog in IFM mode"]
pub type ConfigValueIfmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Configuration value to be passed to analog in IFM mode"]
    #[inline(always)]
    pub fn config_value_ifm(&self) -> ConfigValueIfmR {
        ConfigValueIfmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Configuration value to be passed to analog in IFM mode"]
    #[inline(always)]
    #[must_use]
    pub fn config_value_ifm(&mut self) -> ConfigValueIfmW<Reg2Spec> {
        ConfigValueIfmW::new(self, 0)
    }
}
#[doc = "gpadc config for IFM\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg2Spec;
impl crate::RegisterSpec for Reg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg2::R`](R) reader structure"]
impl crate::Readable for Reg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg2::W`](W) writer structure"]
impl crate::Writable for Reg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG2 to value 0"]
impl crate::Resettable for Reg2Spec {
    const RESET_VALUE: u32 = 0;
}
