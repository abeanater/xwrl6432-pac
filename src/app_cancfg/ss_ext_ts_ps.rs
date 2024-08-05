#[doc = "Register `SS_EXT_TS_PS` reader"]
pub type R = crate::R<SsExtTsPsSpec>;
#[doc = "Register `SS_EXT_TS_PS` writer"]
pub type W = crate::W<SsExtTsPsSpec>;
#[doc = "Field `PRESCALE` reader - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1 ."]
pub type PrescaleR = crate::FieldReader<u32>;
#[doc = "Field `PRESCALE` writer - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1 ."]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU8` reader - 31:24\\]
Reserved"]
pub type Nu8R = crate::FieldReader;
#[doc = "Field `NU8` writer - 31:24\\]
Reserved"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1 ."]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1 ."]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PrescaleW<SsExtTsPsSpec> {
        PrescaleW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<SsExtTsPsSpec> {
        Nu8W::new(self, 24)
    }
}
#[doc = "SS_EXT_TS_PS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ext_ts_ps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ext_ts_ps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsExtTsPsSpec;
impl crate::RegisterSpec for SsExtTsPsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_ext_ts_ps::R`](R) reader structure"]
impl crate::Readable for SsExtTsPsSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_ext_ts_ps::W`](W) writer structure"]
impl crate::Writable for SsExtTsPsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_EXT_TS_PS to value 0"]
impl crate::Resettable for SsExtTsPsSpec {
    const RESET_VALUE: u32 = 0;
}
