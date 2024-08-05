#[doc = "Register `SRC` reader"]
pub type R = crate::R<SrcSpec>;
#[doc = "Register `SRC` writer"]
pub type W = crate::W<SrcSpec>;
#[doc = "Field `SRC` reader - 31:0\\]
Source Address: The 32-bit source address parameters specify the starting byte address of the source . If SAM is set to FIFO mode then the user should program the Source address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
pub type SrcR = crate::FieldReader<u32>;
#[doc = "Field `SRC` writer - 31:0\\]
Source Address: The 32-bit source address parameters specify the starting byte address of the source . If SAM is set to FIFO mode then the user should program the Source address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source Address: The 32-bit source address parameters specify the starting byte address of the source . If SAM is set to FIFO mode then the user should program the Source address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source Address: The 32-bit source address parameters specify the starting byte address of the source . If SAM is set to FIFO mode then the user should program the Source address to be aligned to the value specified by the OPT.FWID field. No errors are recognized here but TC will assert error if this is not true."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<SrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcSpec;
impl crate::RegisterSpec for SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src::R`](R) reader structure"]
impl crate::Readable for SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`src::W`](W) writer structure"]
impl crate::Writable for SrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SrcSpec {
    const RESET_VALUE: u32 = 0;
}
