#[doc = "Register `intr_enable_clear` reader"]
pub type R = crate::R<IntrEnableClearSpec>;
#[doc = "Register `intr_enable_clear` writer"]
pub type W = crate::W<IntrEnableClearSpec>;
#[doc = "Field `PROTECTION_VIOLATION_ERROR` reader - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtectionViolationErrorR = crate::BitReader;
#[doc = "Field `PROTECTION_VIOLATION_ERROR` writer - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtectionViolationErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSING_VIOLATION_ERROR` reader - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddressingViolationErrorR = crate::BitReader;
#[doc = "Field `ADDRESSING_VIOLATION_ERROR` writer - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddressingViolationErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ACCESS_VIOLATION` reader - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type KickAccessViolationR = crate::BitReader;
#[doc = "Field `KICK_ACCESS_VIOLATION` writer - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type KickAccessViolationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY0_ACCESS_VIOLATION` reader - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type Proxy0AccessViolationR = crate::BitReader;
#[doc = "Field `PROXY0_ACCESS_VIOLATION` writer - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type Proxy0AccessViolationW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn protection_violation_error(&self) -> ProtectionViolationErrorR {
        ProtectionViolationErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addressing_violation_error(&self) -> AddressingViolationErrorR {
        AddressingViolationErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_access_violation(&self) -> KickAccessViolationR {
        KickAccessViolationR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy0_access_violation(&self) -> Proxy0AccessViolationR {
        Proxy0AccessViolationR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn protection_violation_error(&mut self) -> ProtectionViolationErrorW<IntrEnableClearSpec> {
        ProtectionViolationErrorW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addressing_violation_error(&mut self) -> AddressingViolationErrorW<IntrEnableClearSpec> {
        AddressingViolationErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_access_violation(&mut self) -> KickAccessViolationW<IntrEnableClearSpec> {
        KickAccessViolationW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy0_access_violation(&mut self) -> Proxy0AccessViolationW<IntrEnableClearSpec> {
        Proxy0AccessViolationW::new(self, 3)
    }
}
#[doc = "Interrupt Enable Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_enable_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_enable_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnableClearSpec;
impl crate::RegisterSpec for IntrEnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_enable_clear::R`](R) reader structure"]
impl crate::Readable for IntrEnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_enable_clear::W`](W) writer structure"]
impl crate::Writable for IntrEnableClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets intr_enable_clear to value 0"]
impl crate::Resettable for IntrEnableClearSpec {
    const RESET_VALUE: u32 = 0;
}
