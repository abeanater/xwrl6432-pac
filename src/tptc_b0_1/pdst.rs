#[doc = "Register `PDST` reader"]
pub type R = crate::R<PdstSpec>;
#[doc = "Register `PDST` writer"]
pub type W = crate::W<PdstSpec>;
#[doc = "Field `DESTINATION_ADDRESS_FOR` reader - 31:0\\]
Destination address for Program Register Set"]
pub type DestinationAddressForR = crate::FieldReader<u32>;
#[doc = "Field `DESTINATION_ADDRESS_FOR` writer - 31:0\\]
Destination address for Program Register Set"]
pub type DestinationAddressForW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Destination address for Program Register Set"]
    #[inline(always)]
    pub fn destination_address_for(&self) -> DestinationAddressForR {
        DestinationAddressForR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Destination address for Program Register Set"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_for(&mut self) -> DestinationAddressForW<PdstSpec> {
        DestinationAddressForW::new(self, 0)
    }
}
#[doc = "Prog Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pdst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdstSpec;
impl crate::RegisterSpec for PdstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdst::R`](R) reader structure"]
impl crate::Readable for PdstSpec {}
#[doc = "`write(|w| ..)` method takes [`pdst::W`](W) writer structure"]
impl crate::Writable for PdstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDST to value 0"]
impl crate::Resettable for PdstSpec {
    const RESET_VALUE: u32 = 0;
}
