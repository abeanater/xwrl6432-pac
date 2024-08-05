#[doc = "Register `REG0` reader"]
pub type R = crate::R<Reg0Spec>;
#[doc = "Register `REG0` writer"]
pub type W = crate::W<Reg0Spec>;
#[doc = "Field `DCBIST_MODE` reader - 1:0\\]
0:Disable,1:IFM Mode enable ,2:CTM mode enable"]
pub type DcbistModeR = crate::FieldReader;
#[doc = "Field `DCBIST_MODE` writer - 1:0\\]
0:Disable,1:IFM Mode enable ,2:CTM mode enable"]
pub type DcbistModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU1` reader - 7:2\\]
TI reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:2\\]
TI reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GPADC_FSM_CLK_ENABLE` reader - 8:8\\]
Enable the clock to gpadc fsm"]
pub type GpadcFsmClkEnableR = crate::BitReader;
#[doc = "Field `GPADC_FSM_CLK_ENABLE` writer - 8:8\\]
Enable the clock to gpadc fsm"]
pub type GpadcFsmClkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPADC2ADCBUF_PATH_EN` reader - 11:9\\]
TI reserved"]
pub type Gpadc2adcbufPathEnR = crate::FieldReader;
#[doc = "Field `GPADC2ADCBUF_PATH_EN` writer - 11:9\\]
TI reserved"]
pub type Gpadc2adcbufPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU2` reader - 15:12\\]
TI reserved"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:12\\]
TI reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPADC_DEBUG_MODE_ENABLE` reader - 16:16\\]
1:GPADC raw samples will be collected in the Output RAM in IFM mode"]
pub type GpadcDebugModeEnableR = crate::BitReader;
#[doc = "Field `GPADC_DEBUG_MODE_ENABLE` writer - 16:16\\]
1:GPADC raw samples will be collected in the Output RAM in IFM mode"]
pub type GpadcDebugModeEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 31:17\\]
TI reserved"]
pub type Nu3R = crate::FieldReader<u16>;
#[doc = "Field `NU3` writer - 31:17\\]
TI reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0:Disable,1:IFM Mode enable ,2:CTM mode enable"]
    #[inline(always)]
    pub fn dcbist_mode(&self) -> DcbistModeR {
        DcbistModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
TI reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable the clock to gpadc fsm"]
    #[inline(always)]
    pub fn gpadc_fsm_clk_enable(&self) -> GpadcFsmClkEnableR {
        GpadcFsmClkEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
TI reserved"]
    #[inline(always)]
    pub fn gpadc2adcbuf_path_en(&self) -> Gpadc2adcbufPathEnR {
        Gpadc2adcbufPathEnR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
TI reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
1:GPADC raw samples will be collected in the Output RAM in IFM mode"]
    #[inline(always)]
    pub fn gpadc_debug_mode_enable(&self) -> GpadcDebugModeEnableR {
        GpadcDebugModeEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
TI reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0:Disable,1:IFM Mode enable ,2:CTM mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcbist_mode(&mut self) -> DcbistModeW<Reg0Spec> {
        DcbistModeW::new(self, 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Reg0Spec> {
        Nu1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable the clock to gpadc fsm"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fsm_clk_enable(&mut self) -> GpadcFsmClkEnableW<Reg0Spec> {
        GpadcFsmClkEnableW::new(self, 8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc2adcbuf_path_en(&mut self) -> Gpadc2adcbufPathEnW<Reg0Spec> {
        Gpadc2adcbufPathEnW::new(self, 9)
    }
    #[doc = "Bits 12:15 - 15:12\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Reg0Spec> {
        Nu2W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
1:GPADC raw samples will be collected in the Output RAM in IFM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_debug_mode_enable(&mut self) -> GpadcDebugModeEnableW<Reg0Spec> {
        GpadcDebugModeEnableW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Reg0Spec> {
        Nu3W::new(self, 17)
    }
}
#[doc = "gpadc modes and enable\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg0Spec;
impl crate::RegisterSpec for Reg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg0::R`](R) reader structure"]
impl crate::Readable for Reg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg0::W`](W) writer structure"]
impl crate::Writable for Reg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for Reg0Spec {
    const RESET_VALUE: u32 = 0;
}
