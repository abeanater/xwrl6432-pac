#[doc = "Register `fault_address` reader"]
pub type R = crate::R<FaultAddressSpec>;
#[doc = "Register `fault_address` writer"]
pub type W = crate::W<FaultAddressSpec>;
#[doc = "Field `FAULT_ADDRESS_` reader - 31:0\\]
Fault Address."]
pub type FaultAddress_R = crate::FieldReader<u32>;
#[doc = "Field `FAULT_ADDRESS_` writer - 31:0\\]
Fault Address."]
pub type FaultAddress_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address."]
    #[inline(always)]
    pub fn fault_address_(&self) -> FaultAddress_R {
        FaultAddress_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address."]
    #[inline(always)]
    #[must_use]
    pub fn fault_address_(&mut self) -> FaultAddress_W<FaultAddressSpec> {
        FaultAddress_W::new(self, 0)
    }
}
#[doc = "Fault Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultAddressSpec;
impl crate::RegisterSpec for FaultAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_address::R`](R) reader structure"]
impl crate::Readable for FaultAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_address::W`](W) writer structure"]
impl crate::Writable for FaultAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fault_address to value 0"]
impl crate::Resettable for FaultAddressSpec {
    const RESET_VALUE: u32 = 0;
}
