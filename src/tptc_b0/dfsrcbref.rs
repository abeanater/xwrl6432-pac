#[doc = "Register `DFSRCBREF` reader"]
pub type R = crate::R<DfsrcbrefSpec>;
#[doc = "Register `DFSRCBREF` writer"]
pub type W = crate::W<DfsrcbrefSpec>;
#[doc = "Field `SOURCE_ADDRESS_REFERENCE_1` reader - 31:0\\]
Source address reference for Destination FIFO Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
pub type SourceAddressReference1R = crate::FieldReader<u32>;
#[doc = "Field `SOURCE_ADDRESS_REFERENCE_1` writer - 31:0\\]
Source address reference for Destination FIFO Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
pub type SourceAddressReference1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source address reference for Destination FIFO Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
    #[inline(always)]
    pub fn source_address_reference_1(&self) -> SourceAddressReference1R {
        SourceAddressReference1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source address reference for Destination FIFO Register Set:#br#Represents the starting address for the array currently being read. The next array's starting address is calculated as the 'reference address' plus the 'source b-idx' value."]
    #[inline(always)]
    #[must_use]
    pub fn source_address_reference_1(&mut self) -> SourceAddressReference1W<DfsrcbrefSpec> {
        SourceAddressReference1W::new(self, 0)
    }
}
#[doc = "Dst FIFO Set Src Addr B-Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsrcbref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsrcbref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsrcbrefSpec;
impl crate::RegisterSpec for DfsrcbrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsrcbref::R`](R) reader structure"]
impl crate::Readable for DfsrcbrefSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsrcbref::W`](W) writer structure"]
impl crate::Writable for DfsrcbrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSRCBREF to value 0"]
impl crate::Resettable for DfsrcbrefSpec {
    const RESET_VALUE: u32 = 0;
}
