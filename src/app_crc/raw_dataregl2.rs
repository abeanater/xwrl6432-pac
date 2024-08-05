#[doc = "Register `RAW_DATAREGL2` reader"]
pub type R = crate::R<RawDataregl2Spec>;
#[doc = "Register `RAW_DATAREGL2` writer"]
pub type W = crate::W<RawDataregl2Spec>;
#[doc = "Field `RAW_DATA2_31_0` reader - 31:0\\]
Channel 2 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
pub type RawData2_31_0R = crate::FieldReader<u32>;
#[doc = "Field `RAW_DATA2_31_0` writer - 31:0\\]
Channel 2 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
pub type RawData2_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
    #[inline(always)]
    pub fn raw_data2_31_0(&self) -> RawData2_31_0R {
        RawData2_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 Raw Data Low Register. This register contains bit 31:0 of the un-compressed raw data."]
    #[inline(always)]
    #[must_use]
    pub fn raw_data2_31_0(&mut self) -> RawData2_31_0W<RawDataregl2Spec> {
        RawData2_31_0W::new(self, 0)
    }
}
#[doc = "Channel 2 un-compressed raw data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDataregl2Spec;
impl crate::RegisterSpec for RawDataregl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dataregl2::R`](R) reader structure"]
impl crate::Readable for RawDataregl2Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_dataregl2::W`](W) writer structure"]
impl crate::Writable for RawDataregl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_DATAREGL2 to value 0"]
impl crate::Resettable for RawDataregl2Spec {
    const RESET_VALUE: u32 = 0;
}
