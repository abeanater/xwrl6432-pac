#[doc = "Register `SASRC` reader"]
pub type R = crate::R<SasrcSpec>;
#[doc = "Register `SASRC` writer"]
pub type W = crate::W<SasrcSpec>;
#[doc = "Field `SOURCE_ADDRESS_FOR_1` reader - 31:0\\]
Source address for Source Active Register Set"]
pub type SourceAddressFor1R = crate::FieldReader<u32>;
#[doc = "Field `SOURCE_ADDRESS_FOR_1` writer - 31:0\\]
Source address for Source Active Register Set"]
pub type SourceAddressFor1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source address for Source Active Register Set"]
    #[inline(always)]
    pub fn source_address_for_1(&self) -> SourceAddressFor1R {
        SourceAddressFor1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source address for Source Active Register Set"]
    #[inline(always)]
    #[must_use]
    pub fn source_address_for_1(&mut self) -> SourceAddressFor1W<SasrcSpec> {
        SourceAddressFor1W::new(self, 0)
    }
}
#[doc = "Src Actv Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`sasrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sasrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SasrcSpec;
impl crate::RegisterSpec for SasrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sasrc::R`](R) reader structure"]
impl crate::Readable for SasrcSpec {}
#[doc = "`write(|w| ..)` method takes [`sasrc::W`](W) writer structure"]
impl crate::Writable for SasrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SASRC to value 0"]
impl crate::Resettable for SasrcSpec {
    const RESET_VALUE: u32 = 0;
}
