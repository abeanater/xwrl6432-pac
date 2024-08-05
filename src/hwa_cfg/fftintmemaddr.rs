#[doc = "Register `FFTINTMEMADDR` reader"]
pub type R = crate::R<FftintmemaddrSpec>;
#[doc = "Register `FFTINTMEMADDR` writer"]
pub type W = crate::W<FftintmemaddrSpec>;
#[doc = "Field `FFT_INT_MEM_ADDR` reader - 8:0\\]
Reserved.TI internal"]
pub type FftIntMemAddrR = crate::FieldReader<u16>;
#[doc = "Field `FFT_INT_MEM_ADDR` writer - 8:0\\]
Reserved.TI internal"]
pub type FftIntMemAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FFT_INT_MEM_SEL` reader - 11:9\\]
Reserved.TI internal"]
pub type FftIntMemSelR = crate::FieldReader;
#[doc = "Field `FFT_INT_MEM_SEL` writer - 11:9\\]
Reserved.TI internal"]
pub type FftIntMemSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU1` reader - 15:12\\]
Reserved.TI internal"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 15:12\\]
Reserved.TI internal"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FFT_INT_MEM_EN` reader - 16:16\\]
Reserved.TI internal"]
pub type FftIntMemEnR = crate::BitReader;
#[doc = "Field `FFT_INT_MEM_EN` writer - 16:16\\]
Reserved.TI internal"]
pub type FftIntMemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 23:17\\]
Reserved.TI internal"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 23:17\\]
Reserved.TI internal"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FFT_INT_MEM_RD` reader - 24:24\\]
Reserved.TI internal"]
pub type FftIntMemRdR = crate::BitReader;
#[doc = "Field `FFT_INT_MEM_RD` writer - 24:24\\]
Reserved.TI internal"]
pub type FftIntMemRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 31:25\\]
Reserved.TI internal"]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 31:25\\]
Reserved.TI internal"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn fft_int_mem_addr(&self) -> FftIntMemAddrR {
        FftIntMemAddrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn fft_int_mem_sel(&self) -> FftIntMemSelR {
        FftIntMemSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn fft_int_mem_en(&self) -> FftIntMemEnR {
        FftIntMemEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn fft_int_mem_rd(&self) -> FftIntMemRdR {
        FftIntMemRdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn fft_int_mem_addr(&mut self) -> FftIntMemAddrW<FftintmemaddrSpec> {
        FftIntMemAddrW::new(self, 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn fft_int_mem_sel(&mut self) -> FftIntMemSelW<FftintmemaddrSpec> {
        FftIntMemSelW::new(self, 9)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<FftintmemaddrSpec> {
        Nu1W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn fft_int_mem_en(&mut self) -> FftIntMemEnW<FftintmemaddrSpec> {
        FftIntMemEnW::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<FftintmemaddrSpec> {
        Nu2W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn fft_int_mem_rd(&mut self) -> FftIntMemRdW<FftintmemaddrSpec> {
        FftIntMemRdW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<FftintmemaddrSpec> {
        Nu3W::new(self, 25)
    }
}
#[doc = "FFTINTMEMADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`fftintmemaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftintmemaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftintmemaddrSpec;
impl crate::RegisterSpec for FftintmemaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fftintmemaddr::R`](R) reader structure"]
impl crate::Readable for FftintmemaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`fftintmemaddr::W`](W) writer structure"]
impl crate::Writable for FftintmemaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFTINTMEMADDR to value 0"]
impl crate::Resettable for FftintmemaddrSpec {
    const RESET_VALUE: u32 = 0;
}
