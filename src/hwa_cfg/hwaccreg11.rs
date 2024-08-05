#[doc = "Register `HWACCREG11` reader"]
pub type R = crate::R<Hwaccreg11Spec>;
#[doc = "Register `HWACCREG11` writer"]
pub type W = crate::W<Hwaccreg11Spec>;
#[doc = "Field `LFSRSEED` reader - 28:0\\]
LFSR seed value (random pattern) for twiddle factor dithering,"]
pub type LfsrseedR = crate::FieldReader<u32>;
#[doc = "Field `LFSRSEED` writer - 28:0\\]
LFSR seed value (random pattern) for twiddle factor dithering,"]
pub type LfsrseedW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LFSRLOAD` reader - 31:31\\]
To load the LFSR seed, a pulse signal needs to be provided, by writing a 1 to the LFSR_LOAD register-bit. Self clearing"]
pub type LfsrloadR = crate::BitReader;
#[doc = "Field `LFSRLOAD` writer - 31:31\\]
To load the LFSR seed, a pulse signal needs to be provided, by writing a 1 to the LFSR_LOAD register-bit. Self clearing"]
pub type LfsrloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
LFSR seed value (random pattern) for twiddle factor dithering,"]
    #[inline(always)]
    pub fn lfsrseed(&self) -> LfsrseedR {
        LfsrseedR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
To load the LFSR seed, a pulse signal needs to be provided, by writing a 1 to the LFSR_LOAD register-bit. Self clearing"]
    #[inline(always)]
    pub fn lfsrload(&self) -> LfsrloadR {
        LfsrloadR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
LFSR seed value (random pattern) for twiddle factor dithering,"]
    #[inline(always)]
    #[must_use]
    pub fn lfsrseed(&mut self) -> LfsrseedW<Hwaccreg11Spec> {
        LfsrseedW::new(self, 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Hwaccreg11Spec> {
        NuW::new(self, 29)
    }
    #[doc = "Bit 31 - 31:31\\]
To load the LFSR seed, a pulse signal needs to be provided, by writing a 1 to the LFSR_LOAD register-bit. Self clearing"]
    #[inline(always)]
    #[must_use]
    pub fn lfsrload(&mut self) -> LfsrloadW<Hwaccreg11Spec> {
        LfsrloadW::new(self, 31)
    }
}
#[doc = "HWACCREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg11Spec;
impl crate::RegisterSpec for Hwaccreg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg11::R`](R) reader structure"]
impl crate::Readable for Hwaccreg11Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg11::W`](W) writer structure"]
impl crate::Writable for Hwaccreg11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG11 to value 0"]
impl crate::Resettable for Hwaccreg11Spec {
    const RESET_VALUE: u32 = 0;
}
