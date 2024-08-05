#[doc = "Register `DFDST1` reader"]
pub type R = crate::R<Dfdst1Spec>;
#[doc = "Register `DFDST1` writer"]
pub type W = crate::W<Dfdst1Spec>;
#[doc = "Field `DESTINATION_ADDRESS_FOR_3` reader - 31:0\\]
Destination address for Dst FIFO Register Set:#br#Initial value is copied from PDST.DADDR.#br#TC updates value according to destination addressing mode \\[OPT.SAM\\]
and/or dest index value \\[BIDX.DBIDX\\]
after each write command is issued.#br#When a TR is complete the final value should be the address of the last write command issued."]
pub type DestinationAddressFor3R = crate::FieldReader<u32>;
#[doc = "Field `DESTINATION_ADDRESS_FOR_3` writer - 31:0\\]
Destination address for Dst FIFO Register Set:#br#Initial value is copied from PDST.DADDR.#br#TC updates value according to destination addressing mode \\[OPT.SAM\\]
and/or dest index value \\[BIDX.DBIDX\\]
after each write command is issued.#br#When a TR is complete the final value should be the address of the last write command issued."]
pub type DestinationAddressFor3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Destination address for Dst FIFO Register Set:#br#Initial value is copied from PDST.DADDR.#br#TC updates value according to destination addressing mode \\[OPT.SAM\\]
and/or dest index value \\[BIDX.DBIDX\\]
after each write command is issued.#br#When a TR is complete the final value should be the address of the last write command issued."]
    #[inline(always)]
    pub fn destination_address_for_3(&self) -> DestinationAddressFor3R {
        DestinationAddressFor3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Destination address for Dst FIFO Register Set:#br#Initial value is copied from PDST.DADDR.#br#TC updates value according to destination addressing mode \\[OPT.SAM\\]
and/or dest index value \\[BIDX.DBIDX\\]
after each write command is issued.#br#When a TR is complete the final value should be the address of the last write command issued."]
    #[inline(always)]
    #[must_use]
    pub fn destination_address_for_3(&mut self) -> DestinationAddressFor3W<Dfdst1Spec> {
        DestinationAddressFor3W::new(self, 0)
    }
}
#[doc = "Dst FIFO Set Dst Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfdst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfdst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfdst1Spec;
impl crate::RegisterSpec for Dfdst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfdst1::R`](R) reader structure"]
impl crate::Readable for Dfdst1Spec {}
#[doc = "`write(|w| ..)` method takes [`dfdst1::W`](W) writer structure"]
impl crate::Writable for Dfdst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFDST1 to value 0"]
impl crate::Resettable for Dfdst1Spec {
    const RESET_VALUE: u32 = 0;
}
