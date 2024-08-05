#[doc = "Register `ADCBUFCFG3` reader"]
pub type R = crate::R<Adcbufcfg3Spec>;
#[doc = "Register `ADCBUFCFG3` writer"]
pub type W = crate::W<Adcbufcfg3Spec>;
#[doc = "Field `ADCBUFADDRX2` reader - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx2 writes in Non-interleaved mode."]
pub type Adcbufaddrx2R = crate::FieldReader<u16>;
#[doc = "Field `ADCBUFADDRX2` writer - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx2 writes in Non-interleaved mode."]
pub type Adcbufaddrx2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ADCBUFADDRX3` reader - 26:16\\]
TI Reserved"]
pub type Adcbufaddrx3R = crate::FieldReader<u16>;
#[doc = "Field `ADCBUFADDRX3` writer - 26:16\\]
TI Reserved"]
pub type Adcbufaddrx3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx2 writes in Non-interleaved mode."]
    #[inline(always)]
    pub fn adcbufaddrx2(&self) -> Adcbufaddrx2R {
        Adcbufaddrx2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
TI Reserved"]
    #[inline(always)]
    pub fn adcbufaddrx3(&self) -> Adcbufaddrx3R {
        Adcbufaddrx3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx2 writes in Non-interleaved mode."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufaddrx2(&mut self) -> Adcbufaddrx2W<Adcbufcfg3Spec> {
        Adcbufaddrx2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
TI Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufaddrx3(&mut self) -> Adcbufaddrx3W<Adcbufcfg3Spec> {
        Adcbufaddrx3W::new(self, 16)
    }
}
#[doc = "ADCBUFCFG3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcbufcfg3Spec;
impl crate::RegisterSpec for Adcbufcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbufcfg3::R`](R) reader structure"]
impl crate::Readable for Adcbufcfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`adcbufcfg3::W`](W) writer structure"]
impl crate::Writable for Adcbufcfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFCFG3 to value 0"]
impl crate::Resettable for Adcbufcfg3Spec {
    const RESET_VALUE: u32 = 0;
}
