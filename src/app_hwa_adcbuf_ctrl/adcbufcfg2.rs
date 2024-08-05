#[doc = "Register `ADCBUFCFG2` reader"]
pub type R = crate::R<Adcbufcfg2Spec>;
#[doc = "Register `ADCBUFCFG2` writer"]
pub type W = crate::W<Adcbufcfg2Spec>;
#[doc = "Field `ADCBUFADDRX0` reader - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx0 writes in Non-interleaved mode."]
pub type Adcbufaddrx0R = crate::FieldReader<u16>;
#[doc = "Field `ADCBUFADDRX0` writer - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx0 writes in Non-interleaved mode."]
pub type Adcbufaddrx0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ADCBUFADDRX1` reader - 26:16\\]
128 bit Address offset to be added to the internal address pointer for Rx1 writes in Non-interleaved mode."]
pub type Adcbufaddrx1R = crate::FieldReader<u16>;
#[doc = "Field `ADCBUFADDRX1` writer - 26:16\\]
128 bit Address offset to be added to the internal address pointer for Rx1 writes in Non-interleaved mode."]
pub type Adcbufaddrx1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx0 writes in Non-interleaved mode."]
    #[inline(always)]
    pub fn adcbufaddrx0(&self) -> Adcbufaddrx0R {
        Adcbufaddrx0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
128 bit Address offset to be added to the internal address pointer for Rx1 writes in Non-interleaved mode."]
    #[inline(always)]
    pub fn adcbufaddrx1(&self) -> Adcbufaddrx1R {
        Adcbufaddrx1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
128 bit Address offset to be added to the internal address pointer for Rx0 writes in Non-interleaved mode."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufaddrx0(&mut self) -> Adcbufaddrx0W<Adcbufcfg2Spec> {
        Adcbufaddrx0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
128 bit Address offset to be added to the internal address pointer for Rx1 writes in Non-interleaved mode."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufaddrx1(&mut self) -> Adcbufaddrx1W<Adcbufcfg2Spec> {
        Adcbufaddrx1W::new(self, 16)
    }
}
#[doc = "ADCBUFCFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcbufcfg2Spec;
impl crate::RegisterSpec for Adcbufcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbufcfg2::R`](R) reader structure"]
impl crate::Readable for Adcbufcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcbufcfg2::W`](W) writer structure"]
impl crate::Writable for Adcbufcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFCFG2 to value 0"]
impl crate::Resettable for Adcbufcfg2Spec {
    const RESET_VALUE: u32 = 0;
}
