#[doc = "Register `DFSRC1` reader"]
pub type R = crate::R<Dfsrc1Spec>;
#[doc = "Register `DFSRC1` writer"]
pub type W = crate::W<Dfsrc1Spec>;
#[doc = "Field `SOURCE_ADDRESS_IS_1` reader - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
pub type SourceAddressIs1R = crate::FieldReader<u32>;
#[doc = "Field `SOURCE_ADDRESS_IS_1` writer - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
pub type SourceAddressIs1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
    #[inline(always)]
    pub fn source_address_is_1(&self) -> SourceAddressIs1R {
        SourceAddressIs1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Source address is not applicable for Dst FIFO Register Set: Reads return 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn source_address_is_1(&mut self) -> SourceAddressIs1W<Dfsrc1Spec> {
        SourceAddressIs1W::new(self, 0)
    }
}
#[doc = "Dst FIFO Set Src Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfsrc1Spec;
impl crate::RegisterSpec for Dfsrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsrc1::R`](R) reader structure"]
impl crate::Readable for Dfsrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`dfsrc1::W`](W) writer structure"]
impl crate::Writable for Dfsrc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSRC1 to value 0"]
impl crate::Resettable for Dfsrc1Spec {
    const RESET_VALUE: u32 = 0;
}
