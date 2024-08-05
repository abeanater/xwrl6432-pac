#[doc = "Register `HWACCREG7` reader"]
pub type R = crate::R<Hwaccreg7Spec>;
#[doc = "Register `HWACCREG7` writer"]
pub type W = crate::W<Hwaccreg7Spec>;
#[doc = "Field `BPMRATE` reader - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
pub type BpmrateR = crate::FieldReader<u16>;
#[doc = "Field `BPMRATE` writer - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
pub type BpmrateW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DITHERTWIDEN` reader - 16:16\\]
Twiddle factor dithering enable: This register-bit is used to enable/disable dithering of twiddle factors in the FFT."]
pub type DithertwidenR = crate::BitReader;
#[doc = "Field `DITHERTWIDEN` writer - 16:16\\]
Twiddle factor dithering enable: This register-bit is used to enable/disable dithering of twiddle factors in the FFT."]
pub type DithertwidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `STG1LUTSELWR` reader - 24:24\\]
Select Window RAM or Internal RAM: The Internal RAM for Vector Multiplication mode is mapped to the same address space as the Window RAM. Hence, this register bit is required to specify which of these two needs to be selected, when loading the co-efficients via DMA or M4. 0 - Window RAM is selected 1 - Internal RAM for Vector Multiplication mode is selected. Keep this register bit as 0 always, except during the period when Internal RAM needs to be loaded."]
pub type Stg1lutselwrR = crate::BitReader;
#[doc = "Field `STG1LUTSELWR` writer - 24:24\\]
Select Window RAM or Internal RAM: The Internal RAM for Vector Multiplication mode is mapped to the same address space as the Window RAM. Hence, this register bit is required to specify which of these two needs to be selected, when loading the co-efficients via DMA or M4. 0 - Window RAM is selected 1 - Internal RAM for Vector Multiplication mode is selected. Keep this register bit as 0 always, except during the period when Internal RAM needs to be loaded."]
pub type Stg1lutselwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - "]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - "]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
    #[inline(always)]
    pub fn bpmrate(&self) -> BpmrateR {
        BpmrateR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Twiddle factor dithering enable: This register-bit is used to enable/disable dithering of twiddle factors in the FFT."]
    #[inline(always)]
    pub fn dithertwiden(&self) -> DithertwidenR {
        DithertwidenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Select Window RAM or Internal RAM: The Internal RAM for Vector Multiplication mode is mapped to the same address space as the Window RAM. Hence, this register bit is required to specify which of these two needs to be selected, when loading the co-efficients via DMA or M4. 0 - Window RAM is selected 1 - Internal RAM for Vector Multiplication mode is selected. Keep this register bit as 0 always, except during the period when Internal RAM needs to be loaded."]
    #[inline(always)]
    pub fn stg1lutselwr(&self) -> Stg1lutselwrR {
        Stg1lutselwrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
BPM rate: Specifies the number of input samples corresponding to each BPM bit. Minimum valid value for this register is 1."]
    #[inline(always)]
    #[must_use]
    pub fn bpmrate(&mut self) -> BpmrateW<Hwaccreg7Spec> {
        BpmrateW::new(self, 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Hwaccreg7Spec> {
        Nu1W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
Twiddle factor dithering enable: This register-bit is used to enable/disable dithering of twiddle factors in the FFT."]
    #[inline(always)]
    #[must_use]
    pub fn dithertwiden(&mut self) -> DithertwidenW<Hwaccreg7Spec> {
        DithertwidenW::new(self, 16)
    }
    #[doc = "Bits 17:23"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Hwaccreg7Spec> {
        Nu2W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Select Window RAM or Internal RAM: The Internal RAM for Vector Multiplication mode is mapped to the same address space as the Window RAM. Hence, this register bit is required to specify which of these two needs to be selected, when loading the co-efficients via DMA or M4. 0 - Window RAM is selected 1 - Internal RAM for Vector Multiplication mode is selected. Keep this register bit as 0 always, except during the period when Internal RAM needs to be loaded."]
    #[inline(always)]
    #[must_use]
    pub fn stg1lutselwr(&mut self) -> Stg1lutselwrW<Hwaccreg7Spec> {
        Stg1lutselwrW::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Hwaccreg7Spec> {
        Nu3W::new(self, 25)
    }
}
#[doc = "HWACCREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg7Spec;
impl crate::RegisterSpec for Hwaccreg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg7::R`](R) reader structure"]
impl crate::Readable for Hwaccreg7Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg7::W`](W) writer structure"]
impl crate::Writable for Hwaccreg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG7 to value 0"]
impl crate::Resettable for Hwaccreg7Spec {
    const RESET_VALUE: u32 = 0;
}
