#[doc = "Register `SASRCBREF` reader"]
pub type R = crate::R<SasrcbrefSpec>;
#[doc = "Register `SASRCBREF` writer"]
pub type W = crate::W<SasrcbrefSpec>;
#[doc = "Field `SOURCE_ADDRESS_REFERENCE` reader - 31:0\\]
Source address reference for Source Active Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
pub type SourceAddressReferenceR = crate::FieldReader<u32>;
#[doc = "Field `SOURCE_ADDRESS_REFERENCE` writer - 31:0\\]
Source address reference for Source Active Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
pub type SourceAddressReferenceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source address reference for Source Active Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
    #[inline(always)]
    pub fn source_address_reference(&self) -> SourceAddressReferenceR {
        SourceAddressReferenceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source address reference for Source Active Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
    #[inline(always)]
    #[must_use]
    pub fn source_address_reference(&mut self) -> SourceAddressReferenceW<SasrcbrefSpec> {
        SourceAddressReferenceW::new(self, 0)
    }
}
#[doc = "Src Actv Set Src Addr B-Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`sasrcbref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sasrcbref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SasrcbrefSpec;
impl crate::RegisterSpec for SasrcbrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sasrcbref::R`](R) reader structure"]
impl crate::Readable for SasrcbrefSpec {}
#[doc = "`write(|w| ..)` method takes [`sasrcbref::W`](W) writer structure"]
impl crate::Writable for SasrcbrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SASRCBREF to value 0"]
impl crate::Resettable for SasrcbrefSpec {
    const RESET_VALUE: u32 = 0;
}
