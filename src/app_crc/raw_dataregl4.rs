#[doc = "Register `RAW_DATAREGL4` reader"]
pub type R = crate::R<RawDataregl4Spec>;
#[doc = "Register `RAW_DATAREGL4` writer"]
pub type W = crate::W<RawDataregl4Spec>;
#[doc = "Field `NU65` reader - 31:0\\]
Reserved"]
pub type Nu65R = crate::FieldReader<u32>;
#[doc = "Field `NU65` writer - 31:0\\]
Reserved"]
pub type Nu65W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu65(&self) -> Nu65R {
        Nu65R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu65(&mut self) -> Nu65W<RawDataregl4Spec> {
        Nu65W::new(self, 0)
    }
}
#[doc = "Channel 4 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDataregl4Spec;
impl crate::RegisterSpec for RawDataregl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dataregl4::R`](R) reader structure"]
impl crate::Readable for RawDataregl4Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_dataregl4::W`](W) writer structure"]
impl crate::Writable for RawDataregl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_DATAREGL4 to value 0"]
impl crate::Resettable for RawDataregl4Spec {
    const RESET_VALUE: u32 = 0;
}
