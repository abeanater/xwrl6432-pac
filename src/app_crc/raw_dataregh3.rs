#[doc = "Register `RAW_DATAREGH3` reader"]
pub type R = crate::R<RawDataregh3Spec>;
#[doc = "Register `RAW_DATAREGH3` writer"]
pub type W = crate::W<RawDataregh3Spec>;
#[doc = "Field `NU53` reader - 31:0\\]
Reserved"]
pub type Nu53R = crate::FieldReader<u32>;
#[doc = "Field `NU53` writer - 31:0\\]
Reserved"]
pub type Nu53W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu53(&self) -> Nu53R {
        Nu53R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu53(&mut self) -> Nu53W<RawDataregh3Spec> {
        Nu53W::new(self, 0)
    }
}
#[doc = "Channel 3 un-compressed raw data high Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDataregh3Spec;
impl crate::RegisterSpec for RawDataregh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dataregh3::R`](R) reader structure"]
impl crate::Readable for RawDataregh3Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_dataregh3::W`](W) writer structure"]
impl crate::Writable for RawDataregh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_DATAREGH3 to value 0"]
impl crate::Resettable for RawDataregh3Spec {
    const RESET_VALUE: u32 = 0;
}
