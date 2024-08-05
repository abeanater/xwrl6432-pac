#[doc = "Register `RAW_DATAREGH2` reader"]
pub type R = crate::R<RawDataregh2Spec>;
#[doc = "Register `RAW_DATAREGH2` writer"]
pub type W = crate::W<RawDataregh2Spec>;
#[doc = "Field `RAW_DATA2_63_32` reader - 31:0\\]
Channel 2 Raw Data High Register. This register contains bit 63:32 of the un-compressed raw data."]
pub type RawData2_63_32R = crate::FieldReader<u32>;
#[doc = "Field `RAW_DATA2_63_32` writer - 31:0\\]
Channel 2 Raw Data High Register. This register contains bit 63:32 of the un-compressed raw data."]
pub type RawData2_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 Raw Data High Register. This register contains bit 63:32 of the un-compressed raw data."]
    #[inline(always)]
    pub fn raw_data2_63_32(&self) -> RawData2_63_32R {
        RawData2_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 Raw Data High Register. This register contains bit 63:32 of the un-compressed raw data."]
    #[inline(always)]
    #[must_use]
    pub fn raw_data2_63_32(&mut self) -> RawData2_63_32W<RawDataregh2Spec> {
        RawData2_63_32W::new(self, 0)
    }
}
#[doc = "Channel 2 un-compressed raw data high Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_dataregh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_dataregh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDataregh2Spec;
impl crate::RegisterSpec for RawDataregh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dataregh2::R`](R) reader structure"]
impl crate::Readable for RawDataregh2Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_dataregh2::W`](W) writer structure"]
impl crate::Writable for RawDataregh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_DATAREGH2 to value 0"]
impl crate::Resettable for RawDataregh2Spec {
    const RESET_VALUE: u32 = 0;
}
