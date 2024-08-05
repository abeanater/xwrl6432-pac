#[doc = "Register `SADSTBREF` reader"]
pub type R = crate::R<SadstbrefSpec>;
#[doc = "Register `SADSTBREF` writer"]
pub type W = crate::W<SadstbrefSpec>;
#[doc = "Field `DST_ADDRESS_REFERENCE` reader - 31:0\\]
Dst address reference is not applicable for Src Active Register Set. Reads return 0x0."]
pub type DstAddressReferenceR = crate::FieldReader<u32>;
#[doc = "Field `DST_ADDRESS_REFERENCE` writer - 31:0\\]
Dst address reference is not applicable for Src Active Register Set. Reads return 0x0."]
pub type DstAddressReferenceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Dst address reference is not applicable for Src Active Register Set. Reads return 0x0."]
    #[inline(always)]
    pub fn dst_address_reference(&self) -> DstAddressReferenceR {
        DstAddressReferenceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Dst address reference is not applicable for Src Active Register Set. Reads return 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn dst_address_reference(&mut self) -> DstAddressReferenceW<SadstbrefSpec> {
        DstAddressReferenceW::new(self, 0)
    }
}
#[doc = "Src Actv Set Dst Addr B-Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`sadstbref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadstbref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SadstbrefSpec;
impl crate::RegisterSpec for SadstbrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sadstbref::R`](R) reader structure"]
impl crate::Readable for SadstbrefSpec {}
#[doc = "`write(|w| ..)` method takes [`sadstbref::W`](W) writer structure"]
impl crate::Writable for SadstbrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADSTBREF to value 0"]
impl crate::Resettable for SadstbrefSpec {
    const RESET_VALUE: u32 = 0;
}
