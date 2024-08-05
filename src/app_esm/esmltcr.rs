#[doc = "Register `ESMLTCR` reader"]
pub type R = crate::R<EsmltcrSpec>;
#[doc = "Register `ESMLTCR` writer"]
pub type W = crate::W<EsmltcrSpec>;
#[doc = "Field `LTCP` reader - 15:0\\]
ERROR Pin Low-Time Counter 16bit pre-loadable down-counter to control low-time of ERROR pin. The low-time counter is triggered by the peripheral clock (VCLK). Note: Low time counter is set to the default preload value of the ESMLTCPR in the following cases: 1. Reset (power on reset or warm reset) 2. An error occurs 3. User forces an error"]
pub type LtcpR = crate::FieldReader<u16>;
#[doc = "Field `LTCP` writer - 15:0\\]
ERROR Pin Low-Time Counter 16bit pre-loadable down-counter to control low-time of ERROR pin. The low-time counter is triggered by the peripheral clock (VCLK). Note: Low time counter is set to the default preload value of the ESMLTCPR in the following cases: 1. Reset (power on reset or warm reset) 2. An error occurs 3. User forces an error"]
pub type LtcpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ERROR Pin Low-Time Counter 16bit pre-loadable down-counter to control low-time of ERROR pin. The low-time counter is triggered by the peripheral clock (VCLK). Note: Low time counter is set to the default preload value of the ESMLTCPR in the following cases: 1. Reset (power on reset or warm reset) 2. An error occurs 3. User forces an error"]
    #[inline(always)]
    pub fn ltcp(&self) -> LtcpR {
        LtcpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ERROR Pin Low-Time Counter 16bit pre-loadable down-counter to control low-time of ERROR pin. The low-time counter is triggered by the peripheral clock (VCLK). Note: Low time counter is set to the default preload value of the ESMLTCPR in the following cases: 1. Reset (power on reset or warm reset) 2. An error occurs 3. User forces an error"]
    #[inline(always)]
    #[must_use]
    pub fn ltcp(&mut self) -> LtcpW<EsmltcrSpec> {
        LtcpW::new(self, 0)
    }
}
#[doc = "ESM Low-Time Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmltcrSpec;
impl crate::RegisterSpec for EsmltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmltcr::R`](R) reader structure"]
impl crate::Readable for EsmltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`esmltcr::W`](W) writer structure"]
impl crate::Writable for EsmltcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMLTCR to value 0"]
impl crate::Resettable for EsmltcrSpec {
    const RESET_VALUE: u32 = 0;
}
