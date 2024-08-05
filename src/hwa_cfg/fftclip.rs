#[doc = "Register `FFTCLIP` reader"]
pub type R = crate::R<FftclipSpec>;
#[doc = "Register `FFTCLIP` writer"]
pub type W = crate::W<FftclipSpec>;
#[doc = "Field `FFTCLIPSTAT` reader - 9:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the 10 butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate/clip samples. In that case, this saturation event is indicated in the corresponding bit in this status registert. If multiple FFTs are performed, this status register includes any saturation events happening in any of them."]
pub type FftclipstatR = crate::FieldReader<u16>;
#[doc = "Field `FFTCLIPSTAT` writer - 9:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the 10 butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate/clip samples. In that case, this saturation event is indicated in the corresponding bit in this status registert. If multiple FFTs are performed, this status register includes any saturation events happening in any of them."]
pub type FftclipstatW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLRFFTCLIPSTAT` reader - 16:16\\]
FFTCLIPSTAT can be cleared by setting single-bit register CLRFFTCLIPSTAT, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored."]
pub type ClrfftclipstatR = crate::BitReader;
#[doc = "Field `CLRFFTCLIPSTAT` writer - 16:16\\]
FFTCLIPSTAT can be cleared by setting single-bit register CLRFFTCLIPSTAT, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored."]
pub type ClrfftclipstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u16>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the 10 butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate/clip samples. In that case, this saturation event is indicated in the corresponding bit in this status registert. If multiple FFTs are performed, this status register includes any saturation events happening in any of them."]
    #[inline(always)]
    pub fn fftclipstat(&self) -> FftclipstatR {
        FftclipstatR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
FFTCLIPSTAT can be cleared by setting single-bit register CLRFFTCLIPSTAT, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored."]
    #[inline(always)]
    pub fn clrfftclipstat(&self) -> ClrfftclipstatR {
        ClrfftclipstatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
FFT Clip Status (read-only): This is a read-only status register, which indicates any saturation/clipping events that have happened in the FFT butterfly stages. Note that each of the 10 butterfly stages in the FFT can be programmed to either saturate the MSB or round the LSB. Whenever saturation of MSB is used in any stage, there is a possibility that that stage can saturate/clip samples. In that case, this saturation event is indicated in the corresponding bit in this status registert. If multiple FFTs are performed, this status register includes any saturation events happening in any of them."]
    #[inline(always)]
    #[must_use]
    pub fn fftclipstat(&mut self) -> FftclipstatW<FftclipSpec> {
        FftclipstatW::new(self, 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<FftclipSpec> {
        Nu1W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
FFTCLIPSTAT can be cleared by setting single-bit register CLRFFTCLIPSTAT, so that the saturation status indication gets cleared back to 0 and any subsequent saturation events can be freshly monitored."]
    #[inline(always)]
    #[must_use]
    pub fn clrfftclipstat(&mut self) -> ClrfftclipstatW<FftclipSpec> {
        ClrfftclipstatW::new(self, 16)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<FftclipSpec> {
        Nu2W::new(self, 17)
    }
}
#[doc = "FFTCLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`fftclip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftclip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftclipSpec;
impl crate::RegisterSpec for FftclipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fftclip::R`](R) reader structure"]
impl crate::Readable for FftclipSpec {}
#[doc = "`write(|w| ..)` method takes [`fftclip::W`](W) writer structure"]
impl crate::Writable for FftclipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFTCLIP to value 0"]
impl crate::Resettable for FftclipSpec {
    const RESET_VALUE: u32 = 0;
}
