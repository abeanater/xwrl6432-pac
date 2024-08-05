#[doc = "Register `RAW_DATAREGH4` reader"]
pub type R = crate::R<RawDataregh4Spec>;
#[doc = "Register `RAW_DATAREGH4` writer"]
pub type W = crate::W<RawDataregh4Spec>;
#[doc = "Field `NU66` reader - 31:0\\]
Reserved"]
pub type Nu66R = crate::FieldReader<u32>;
#[doc = "Field `NU66` writer - 31:0\\]
Reserved"]
pub type Nu66W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu66(&self) -> Nu66R {
        Nu66R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu66(&mut self) -> Nu66W<RawDataregh4Spec> {
        Nu66W::new(self, 0)
    }
}
#[doc = "Channel 4 un-compressed raw data high Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDataregh4Spec;
impl crate::RegisterSpec for RawDataregh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dataregh4::R`](R) reader structure"]
impl crate::Readable for RawDataregh4Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_dataregh4::W`](W) writer structure"]
impl crate::Writable for RawDataregh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_DATAREGH4 to value 0"]
impl crate::Resettable for RawDataregh4Spec {
    const RESET_VALUE: u32 = 0;
}
