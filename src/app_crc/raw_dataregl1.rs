#[doc = "Register `RAW_DATAREGL1` reader"]
pub type R = crate::R<RawDataregl1Spec>;
#[doc = "Register `RAW_DATAREGL1` writer"]
pub type W = crate::W<RawDataregl1Spec>;
#[doc = "Field `RAW_DATA1_31_0` reader - 31:0\\]
Channel 1 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
pub type RawData1_31_0R = crate::FieldReader<u32>;
#[doc = "Field `RAW_DATA1_31_0` writer - 31:0\\]
Channel 1 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
pub type RawData1_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
    #[inline(always)]
    pub fn raw_data1_31_0(&self) -> RawData1_31_0R {
        RawData1_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
    #[inline(always)]
    #[must_use]
    pub fn raw_data1_31_0(&mut self) -> RawData1_31_0W<RawDataregl1Spec> {
        RawData1_31_0W::new(self, 0)
    }
}
#[doc = "Channel 1 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDataregl1Spec;
impl crate::RegisterSpec for RawDataregl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dataregl1::R`](R) reader structure"]
impl crate::Readable for RawDataregl1Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_dataregl1::W`](W) writer structure"]
impl crate::Writable for RawDataregl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_DATAREGL1 to value 0"]
impl crate::Resettable for RawDataregl1Spec {
    const RESET_VALUE: u32 = 0;
}
