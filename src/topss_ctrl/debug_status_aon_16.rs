#[doc = "Register `DEBUG_STATUS_AON_16` reader"]
pub type R = crate::R<DebugStatusAon16Spec>;
#[doc = "Register `DEBUG_STATUS_AON_16` writer"]
pub type W = crate::W<DebugStatusAon16Spec>;
#[doc = "Field `icemelter_powakeemu` reader - 0:0\\]
status reg for icemelter_powakeemu"]
pub type IcemelterPowakeemuR = crate::BitReader;
#[doc = "Field `icemelter_powakeemu` writer - 0:0\\]
status reg for icemelter_powakeemu"]
pub type IcemelterPowakeemuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for icemelter_powakeemu"]
    #[inline(always)]
    pub fn icemelter_powakeemu(&self) -> IcemelterPowakeemuR {
        IcemelterPowakeemuR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for icemelter_powakeemu"]
    #[inline(always)]
    #[must_use]
    pub fn icemelter_powakeemu(&mut self) -> IcemelterPowakeemuW<DebugStatusAon16Spec> {
        IcemelterPowakeemuW::new(self, 0)
    }
}
#[doc = "DEBUG_STATUS_AON_16\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon16Spec;
impl crate::RegisterSpec for DebugStatusAon16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_16::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon16Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_16::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_16 to value 0"]
impl crate::Resettable for DebugStatusAon16Spec {
    const RESET_VALUE: u32 = 0;
}
