#[doc = "Register `SADST` reader"]
pub type R = crate::R<SadstSpec>;
#[doc = "Register `SADST` writer"]
pub type W = crate::W<SadstSpec>;
#[doc = "Field `DESTINATION_ADDRESS_FOR_1` reader - 31:0\\]
Destination address for Source Active Register Set"]
pub type DestinationAddressFor1R = crate::FieldReader<u32>;
#[doc = "Field `DESTINATION_ADDRESS_FOR_1` writer - 31:0\\]
Destination address for Source Active Register Set"]
pub type DestinationAddressFor1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Destination address for Source Active Register Set"]
    #[inline(always)]
    pub fn destination_address_for_1(&self) -> DestinationAddressFor1R {
        DestinationAddressFor1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Destination address for Source Active Register Set"]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_for_1(&mut self) -> DestinationAddressFor1W<SadstSpec> {
        DestinationAddressFor1W::new(self, 0)
    }
}
#[doc = "Src Actv Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sadst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SadstSpec;
impl crate::RegisterSpec for SadstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sadst::R`](R) reader structure"]
impl crate::Readable for SadstSpec {}
#[doc = "`write(|w| ..)` method takes [`sadst::W`](W) writer structure"]
impl crate::Writable for SadstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADST to value 0"]
impl crate::Resettable for SadstSpec {
    const RESET_VALUE: u32 = 0;
}
