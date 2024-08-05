#[doc = "Register `DFSRC0` reader"]
pub type R = crate::R<Dfsrc0Spec>;
#[doc = "Register `DFSRC0` writer"]
pub type W = crate::W<Dfsrc0Spec>;
#[doc = "Field `SOURCE_ADDRESS_IS` reader - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
pub type SourceAddressIsR = crate::FieldReader<u32>;
#[doc = "Field `SOURCE_ADDRESS_IS` writer - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
pub type SourceAddressIsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
    #[inline(always)]
    pub fn source_address_is(&self) -> SourceAddressIsR {
        SourceAddressIsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn source_address_is(&mut self) -> SourceAddressIsW<Dfsrc0Spec> {
        SourceAddressIsW::new(self, 0)
    }
}
#[doc = "Dst FIFO Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfsrc0Spec;
impl crate::RegisterSpec for Dfsrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsrc0::R`](R) reader structure"]
impl crate::Readable for Dfsrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`dfsrc0::W`](W) writer structure"]
impl crate::Writable for Dfsrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSRC0 to value 0"]
impl crate::Resettable for Dfsrc0Spec {
    const RESET_VALUE: u32 = 0;
}
