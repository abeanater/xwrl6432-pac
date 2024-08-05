#[doc = "Register `intr_raw_status` reader"]
pub type R = crate::R<IntrRawStatusSpec>;
#[doc = "Register `intr_raw_status` writer"]
pub type W = crate::W<IntrRawStatusSpec>;
#[doc = "Field `PROTECTION_VIOLATION_ERROR_` reader - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtectionViolationError_R = crate::BitReader;
#[doc = "Field `PROTECTION_VIOLATION_ERROR_` writer - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtectionViolationError_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSING_VIOLATION_ERROR_` reader - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddressingViolationError_R = crate::BitReader;
#[doc = "Field `ADDRESSING_VIOLATION_ERROR_` writer - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddressingViolationError_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ACCESS_VIOLATION` reader - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type KickAccessViolationR = crate::BitReader;
#[doc = "Field `KICK_ACCESS_VIOLATION` writer - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type KickAccessViolationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY0_ACCESS_VIOLATION` reader - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type Proxy0AccessViolationR = crate::BitReader;
#[doc = "Field `PROXY0_ACCESS_VIOLATION` writer - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type Proxy0AccessViolationW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn protection_violation_error_(&self) -> ProtectionViolationError_R {
        ProtectionViolationError_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addressing_violation_error_(&self) -> AddressingViolationError_R {
        AddressingViolationError_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_access_violation(&self) -> KickAccessViolationR {
        KickAccessViolationR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy0_access_violation(&self) -> Proxy0AccessViolationR {
        Proxy0AccessViolationR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn protection_violation_error_(&mut self) -> ProtectionViolationError_W<IntrRawStatusSpec> {
        ProtectionViolationError_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addressing_violation_error_(&mut self) -> AddressingViolationError_W<IntrRawStatusSpec> {
        AddressingViolationError_W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_access_violation(&mut self) -> KickAccessViolationW<IntrRawStatusSpec> {
        KickAccessViolationW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy0_access_violation(&mut self) -> Proxy0AccessViolationW<IntrRawStatusSpec> {
        Proxy0AccessViolationW::new(self, 3)
    }
}
#[doc = "Interrupt Raw Status/Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_raw_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRawStatusSpec;
impl crate::RegisterSpec for IntrRawStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw_status::R`](R) reader structure"]
impl crate::Readable for IntrRawStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_raw_status::W`](W) writer structure"]
impl crate::Writable for IntrRawStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets intr_raw_status to value 0"]
impl crate::Resettable for IntrRawStatusSpec {
    const RESET_VALUE: u32 = 0;
}
