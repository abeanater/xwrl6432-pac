#[doc = "Register `PSRC` reader"]
pub type R = crate::R<PsrcSpec>;
#[doc = "Register `PSRC` writer"]
pub type W = crate::W<PsrcSpec>;
#[doc = "Field `SOURCE_ADDRESS_FOR` reader - 31:0\\]
Source address for Program Register Set"]
pub type SourceAddressForR = crate::FieldReader<u32>;
#[doc = "Field `SOURCE_ADDRESS_FOR` writer - 31:0\\]
Source address for Program Register Set"]
pub type SourceAddressForW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source address for Program Register Set"]
    #[inline(always)]
    pub fn source_address_for(&self) -> SourceAddressForR {
        SourceAddressForR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source address for Program Register Set"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_for(&mut self) -> SourceAddressForW<PsrcSpec> {
        SourceAddressForW::new(self, 0)
    }
}
#[doc = "Prog Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`psrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrcSpec;
impl crate::RegisterSpec for PsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psrc::R`](R) reader structure"]
impl crate::Readable for PsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`psrc::W`](W) writer structure"]
impl crate::Writable for PsrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSRC to value 0"]
impl crate::Resettable for PsrcSpec {
    const RESET_VALUE: u32 = 0;
}
