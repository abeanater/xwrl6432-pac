#[doc = "Register `DST` reader"]
pub type R = crate::R<DstSpec>;
#[doc = "Register `DST` writer"]
pub type W = crate::W<DstSpec>;
#[doc = "Field `DST` reader - 31:0\\]
Destination Address: The 32-bit destination address parameters specify the starting byte address of the destination. If DAM is set to FIFO mode then the user should program the Destination address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
pub type DstR = crate::FieldReader<u32>;
#[doc = "Field `DST` writer - 31:0\\]
Destination Address: The 32-bit destination address parameters specify the starting byte address of the destination. If DAM is set to FIFO mode then the user should program the Destination address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
pub type DstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Destination Address: The 32-bit destination address parameters specify the starting byte address of the destination. If DAM is set to FIFO mode then the user should program the Destination address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
    #[inline(always)]
    pub fn dst(&self) -> DstR {
        DstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Destination Address: The 32-bit destination address parameters specify the starting byte address of the destination. If DAM is set to FIFO mode then the user should program the Destination address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
    #[inline(always)]
    #[must_use]
    pub fn dst(&mut self) -> DstW<DstSpec> {
        DstW::new(self, 0)
    }
}
#[doc = "Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstSpec;
impl crate::RegisterSpec for DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst::R`](R) reader structure"]
impl crate::Readable for DstSpec {}
#[doc = "`write(|w| ..)` method takes [`dst::W`](W) writer structure"]
impl crate::Writable for DstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST to value 0"]
impl crate::Resettable for DstSpec {
    const RESET_VALUE: u32 = 0;
}
